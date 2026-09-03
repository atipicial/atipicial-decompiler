use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::manifest::ContractManifest;
use crate::aef::AefFile;
use crate::util;

use super::super::super::args::Cli;
use super::super::super::reports::{self, InfoReport, MethodTokenReport};

impl Cli {
    pub(super) fn print_info_json(
        &self,
        path: &Path,
        aef: &AefFile,
        manifest: Option<&ContractManifest>,
        manifest_path: Option<&PathBuf>,
    ) -> Result<()> {
        let script_hash = aef.script_hash();
        let method_tokens: Vec<MethodTokenReport> = aef
            .method_tokens
            .iter()
            .map(reports::build_method_token_report)
            .collect();
        let warnings = reports::collect_warnings(&method_tokens);

        let manifest_summary = manifest.map(reports::summarize_manifest);

        let report = InfoReport {
            file: path.display().to_string(),
            manifest_path: manifest_path.map(|p| p.display().to_string()),
            compiler: aef.header.compiler.clone(),
            source: if aef.header.source.is_empty() {
                None
            } else {
                Some(aef.header.source.clone())
            },
            script_length: aef.script.len(),
            script_hash_le: util::format_hash(&script_hash),
            script_hash_be: util::format_hash_be(&script_hash),
            checksum: format!("0x{:08X}", aef.checksum),
            method_tokens,
            manifest: manifest_summary,
            warnings,
        };

        self.print_json(&report)
    }
}
