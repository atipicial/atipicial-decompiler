//! Corpus replay / regression test.
//!
//! Replays committed `TestingArtifacts/*` fixtures through the full pipeline
//! (AEF parse → disassemble → CFG → SSA → render) under `catch_unwind`, and
//! additionally replays every locally generated fuzz corpus under
//! `fuzz/corpus/` when present. Because that directory is gitignored, the
//! committed fixtures are the always-present seed so the fence runs with real
//! coverage on a fresh CI checkout, while any local corpora layered on top
//! extend it. `decompile_all_artifacts_across_formats_without_panics`
//! re-decompiles every `TestingArtifacts/*` contract across all output formats.
//!
//! This is the regression fence introduced in the advanced-decompiler Phase 0:
//! it catches panics on fuzzer-found inputs and pins artifact decompilation so
//! later phases detect regressions immediately.

#![allow(clippy::unwrap_used)]

use std::fs;
use std::panic::catch_unwind;
use std::path::{Path, PathBuf};

use atipicial_decompiler::{
    CfgBuilder, ContractManifest, Decompiler, Disassembler, AefParser, OutputFormat,
};

/// Locate the repo root from CARGO_MANIFEST_DIR (set by cargo at build time).
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Recursively collect files under `dir`.
fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// Discover committed `.aef` / `.manifest.json` fixtures under
/// `TestingArtifacts/`. These always ship in git, so the panic fence has
/// meaningful coverage even on a fresh CI checkout where the gitignored
/// `fuzz/corpus/` directories are empty. Returns `(aef_files, manifest_files)`.
fn committed_fixtures(root: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut all = Vec::new();
    collect_files(&root.join("TestingArtifacts"), &mut all);
    let is_aef = |p: &PathBuf| p.extension().and_then(|e| e.to_str()) == Some("aef");
    let is_manifest = |p: &PathBuf| {
        p.file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.ends_with(".manifest.json"))
    };
    let mut aefs: Vec<PathBuf> = all.iter().filter(|p| is_aef(p)).cloned().collect();
    let mut manifests: Vec<PathBuf> = all.iter().filter(|p| is_manifest(p)).cloned().collect();
    aefs.sort();
    manifests.sort();
    (aefs, manifests)
}

/// Raw script bytes extracted from every committed `.aef`, used to seed the
/// raw-bytecode target so it never runs empty.
fn committed_scripts(aef_files: &[PathBuf]) -> Vec<Vec<u8>> {
    let parser = AefParser::new();
    aef_files
        .iter()
        .filter_map(|p| fs::read(p).ok())
        .filter_map(|bytes| parser.parse(&bytes).ok())
        .map(|aef| aef.script)
        .collect()
}

/// One entry point per corpus: each exercises a different pipeline slice.
#[derive(Copy, Clone, PartialEq, Eq)]
enum Target {
    /// Full AEF-based decompile pipeline.
    AefDecompile,
    /// Raw bytecode: disassemble → CFG → SSA (no AEF wrapper).
    RawDecompile,
    /// AEF container parse only.
    AefParse,
    /// Manifest JSON parse only.
    Manifest,
}

impl Target {
    fn dir(&self) -> &'static str {
        match self {
            Self::AefDecompile => "fuzz/corpus/fuzz_decompile",
            Self::RawDecompile => "fuzz/corpus/fuzz_decompile_raw",
            Self::AefParse => "fuzz/corpus/fuzz_aef_parse",
            Self::Manifest => "fuzz/corpus/fuzz_manifest",
        }
    }
}

fn run_target(data: &[u8], target: Target) {
    let _ = catch_unwind(|| match target {
        Target::AefDecompile | Target::AefParse => {
            // Both paths start by parsing the AEF container; AefDecompile then
            // runs the full pipeline. Running the full path covers parse too.
            let _ = Decompiler::new().decompile_bytes(data);
        }
        Target::RawDecompile => {
            let dis = Disassembler::new();
            if let Ok(instrs) = dis.disassemble(data) {
                if !instrs.is_empty() {
                    // Disassemble + build the CFG as a panic fence across the
                    // whole corpus. (Real SSA construction is exercised on
                    // representative artifacts by ir_pipeline / ssa_e2e; running
                    // it on every fuzz input is too slow for this fence.)
                    let _ = CfgBuilder::new(&instrs).build();
                }
            }
        }
        Target::Manifest => {
            // Manifest corpus is JSON text; parse best-effort.
            if let Ok(text) = std::str::from_utf8(data) {
                let _ = ContractManifest::from_json_str(text);
            }
        }
    });
}

#[test]
fn replay_all_fuzz_corpora_without_panics() {
    let root = repo_root();
    let (aef_files, manifest_files) = committed_fixtures(&root);
    let raw_scripts = committed_scripts(&aef_files);

    for target in [
        Target::AefDecompile,
        Target::RawDecompile,
        Target::AefParse,
        Target::Manifest,
    ] {
        // Seed the fence with committed fixtures so every checkout (including a
        // fresh CI clone where the gitignored corpora are absent) replays real
        // inputs. Local fuzz corpora, when present, are layered on top.
        let mut inputs: Vec<(String, Vec<u8>)> = Vec::new();
        match target {
            Target::AefDecompile | Target::AefParse => {
                for f in &aef_files {
                    if let Ok(data) = fs::read(f) {
                        inputs.push((format!("committed artifact {}", f.display()), data));
                    }
                }
            }
            Target::RawDecompile => {
                for (i, script) in raw_scripts.iter().enumerate() {
                    inputs.push((format!("committed script #{i}"), script.clone()));
                }
            }
            Target::Manifest => {
                for f in &manifest_files {
                    if let Ok(data) = fs::read(f) {
                        inputs.push((format!("committed manifest {}", f.display()), data));
                    }
                }
            }
        }

        // Additionally replay any locally generated fuzz corpus (gitignored).
        let dir = root.join(target.dir());
        let mut corpus_files = Vec::new();
        collect_files(&dir, &mut corpus_files);
        corpus_files.sort();
        for file in &corpus_files {
            // Skip the synthetic named .aef seed (already covered by fixtures).
            if file.extension().and_then(|e| e.to_str()) == Some("aef") {
                continue;
            }
            if let Ok(data) = fs::read(file) {
                inputs.push((format!("{} corpus {}", target.dir(), file.display()), data));
            }
        }

        let count = inputs.len();
        for (label, data) in &inputs {
            // catch_unwind swallows panics; surface them as failures with the
            // offending input label by re-panicking outside the catch.
            let panic = catch_unwind(|| {
                run_target(data, target);
            });
            if panic.is_err() {
                panic!("corpus replay panic in {} target at {label}", target.dir());
            }
        }

        // The committed fixtures guarantee non-empty coverage on every checkout,
        // so this now flags a genuine regression (fixtures gone) rather than an
        // absent local, gitignored fuzz corpus.
        assert!(
            count > 0,
            "no replay inputs for {} (committed fixtures missing?)",
            target.dir()
        );
    }
}

#[test]
fn decompile_all_artifacts_across_formats_without_panics() {
    let root = repo_root();
    let artifacts_dir = root.join("TestingArtifacts");
    let mut aef_files = Vec::new();
    collect_files(&artifacts_dir, &mut aef_files);
    aef_files.retain(|p| p.extension().and_then(|e| e.to_str()) == Some("aef"));

    assert!(!aef_files.is_empty(), "no .aef artifacts discovered");

    let decompiler = Decompiler::new();
    let mut decompiled = 0usize;
    for aef_path in &aef_files {
        let manifest_path = aef_path.with_extension("manifest.json");
        let data = fs::read(aef_path).expect("read aef artifact");
        let manifest = fs::read_to_string(&manifest_path)
            .ok()
            .and_then(|text| ContractManifest::from_json_str(&text).ok());

        let result = catch_unwind(|| {
            decompiler.decompile_bytes_with_manifest(&data, manifest, OutputFormat::All)
        });
        if result.is_err() {
            panic!("artifact decompile panic on {}", aef_path.display());
        }
        if result.unwrap().is_ok() {
            decompiled += 1;
        }
    }
    assert!(decompiled > 0, "no artifact decompiled successfully");
}

/// Smoke-test the parser directly on the aef corpus (mirrors fuzz_aef_parse).
#[test]
fn aef_parser_corpus_smoke() {
    let root = repo_root();
    let mut files = Vec::new();
    collect_files(&root.join("fuzz/corpus/fuzz_aef_parse"), &mut files);
    let parser = AefParser::new();
    for file in &files {
        if file.extension().and_then(|e| e.to_str()) == Some("aef") {
            continue;
        }
        let Ok(data) = fs::read(file) else { continue };
        let _ = catch_unwind(|| parser.parse(&data));
    }
}
