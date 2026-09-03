use std::path::{Path, PathBuf};

use std::io::Write as _;

use crate::error::Result;
use crate::manifest::ContractManifest;
use crate::aef::AefFile;
use crate::util;

use super::super::super::args::Cli;
use super::super::super::reports;

impl Cli {
    pub(super) fn print_info_text(
        &self,
        path: &Path,
        aef: &AefFile,
        manifest: Option<&ContractManifest>,
        manifest_path: Option<&PathBuf>,
    ) -> Result<()> {
        self.write_stdout(|out| {
            writeln!(out, "File: {}", path.display())?;
            if !aef.header.compiler.is_empty() {
                writeln!(out, "Compiler: {}", aef.header.compiler)?;
            }
            if !aef.header.source.is_empty() {
                writeln!(out, "Source: {}", aef.header.source)?;
            }
            writeln!(out, "Script length: {} bytes", aef.script.len())?;
            let script_hash = aef.script_hash();
            writeln!(out, "Script hash (LE): {}", util::format_hash(&script_hash))?;
            writeln!(
                out,
                "Script hash (BE): {}",
                util::format_hash_be(&script_hash)
            )?;
            writeln!(out, "Method tokens: {}", aef.method_tokens.len())?;
            if !aef.method_tokens.is_empty() {
                writeln!(out, "Method token entries:")?;
                for (index, token) in aef.method_tokens.iter().enumerate() {
                    writeln!(
                        out,
                        "    {}",
                        reports::format_method_token_line(index, token)
                    )?;
                }
            }
            writeln!(out, "Checksum: 0x{:08X}", aef.checksum)?;

            if let Some(manifest) = manifest {
                writeln!(out, "Manifest contract: {}", manifest.name)?;
                if !manifest.supported_standards.is_empty() {
                    writeln!(
                        out,
                        "Supported standards: {}",
                        manifest.supported_standards.join(", ")
                    )?;
                }
                writeln!(out, "ABI methods: {}", manifest.abi.methods.len())?;
                writeln!(out, "ABI events: {}", manifest.abi.events.len())?;
                // Atipicial requires `features` to be empty; only surface it when
                // a (malformed) manifest actually carries content.
                if !manifest.features.is_empty() {
                    writeln!(
                        out,
                        "Features: {}",
                        serde_json::Value::Object(manifest.features.clone())
                    )?;
                }
                if !manifest.groups.is_empty() {
                    writeln!(out, "Groups:")?;
                    for group in &manifest.groups {
                        writeln!(
                            out,
                            "    - pubkey={} signature={}",
                            group.pubkey, group.signature
                        )?;
                    }
                }
                if !manifest.permissions.is_empty() {
                    writeln!(out, "Permissions:")?;
                    for permission in &manifest.permissions {
                        writeln!(
                            out,
                            "    - contract={} methods={}",
                            permission.contract.describe(),
                            permission.methods.describe()
                        )?;
                    }
                }
                if let Some(trusts) = manifest.trusts.as_ref() {
                    writeln!(out, "Trusts: {}", trusts.describe())?;
                }
                if let Some(serde_json::Value::Object(map)) = manifest.extra.as_ref() {
                    // Show only entries we can render without ambiguity —
                    // strings/numbers/booleans. Objects, arrays, and `null`
                    // have no canonical short form and would clutter the
                    // text view; the JSON `info --format json` surface
                    // exposes the raw structure for programmatic consumers.
                    let renderable: Vec<(&String, String)> = map
                        .iter()
                        .filter_map(|(k, v)| match v {
                            serde_json::Value::String(s) => Some((k, s.clone())),
                            serde_json::Value::Bool(b) => Some((k, b.to_string())),
                            serde_json::Value::Number(n) => Some((k, n.to_string())),
                            _ => None,
                        })
                        .collect();
                    if !renderable.is_empty() {
                        writeln!(out, "Extra:")?;
                        for (key, value) in renderable {
                            writeln!(out, "    - {key}: {value}")?;
                        }
                    }
                }
                if let Some(path) = manifest_path {
                    writeln!(out, "Manifest path: {}", path.display())?;
                }
            }
            Ok(())
        })
    }
}
