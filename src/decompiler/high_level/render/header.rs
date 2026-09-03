use std::fmt::Write;

use crate::manifest::ContractManifest;
use crate::aef::AefFile;
use crate::util;

use super::super::super::helpers::{extract_contract_name, sanitize_identifier};
use super::{manifest_summary, method_tokens};

pub(crate) fn write_contract_header(
    output: &mut String,
    aef: &AefFile,
    manifest: Option<&ContractManifest>,
) {
    let contract_name = extract_contract_name(manifest, sanitize_identifier);

    writeln!(output, "contract {contract_name} {{").unwrap();
    let script_hash = aef.script_hash();
    writeln!(
        output,
        "    // script hash (little-endian): {}",
        util::format_hash(&script_hash)
    )
    .unwrap();
    writeln!(
        output,
        "    // script hash (big-endian): {}",
        util::format_hash_be(&script_hash)
    )
    .unwrap();
    if !aef.header.compiler.is_empty() {
        writeln!(output, "    // compiler: {}", aef.header.compiler).unwrap();
    }
    if !aef.header.source.is_empty() {
        writeln!(output, "    // source: {}", aef.header.source).unwrap();
    }

    if let Some(manifest) = manifest {
        manifest_summary::write_manifest_summary(output, manifest);
    } else {
        writeln!(output, "    // manifest not provided").unwrap();
    }

    method_tokens::write_method_tokens(output, aef);

    writeln!(output).unwrap();
}
