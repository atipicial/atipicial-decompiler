use predicates::str::contains;
use serde_json::Value;
use tempfile::tempdir;

use crate::common::{
    assert_schema, build_aef_with_no_tokens, build_sample_aef, atipicial_decompiler_cmd,
    write_oversize_aef, SchemaKind,
};

#[test]
fn tokens_command_lists_entries() {
    let dir = tempdir().expect("tempdir");
    let aef_path = dir.path().join("contract.aef");
    std::fs::write(&aef_path, build_sample_aef()).unwrap();

    atipicial_decompiler_cmd()
        .arg("tokens")
        .arg(&aef_path)
        .assert()
        .success()
        .stdout(contains("method=Transfer"))
        .stdout(contains("AtipicialDollar::Transfer"))
        .stdout(contains("AllowCall"));
}

#[test]
fn tokens_command_supports_json_output() {
    let dir = tempdir().expect("tempdir");
    let aef_path = dir.path().join("contract.aef");
    std::fs::write(&aef_path, build_sample_aef()).unwrap();

    let output = atipicial_decompiler_cmd()
        .arg("tokens")
        .arg("--format")
        .arg("json")
        .arg(&aef_path)
        .output()
        .expect("json output");
    assert!(output.status.success());

    let value: Value = serde_json::from_slice(&output.stdout).expect("json parse");
    let tokens = value["method_tokens"].as_array().expect("array");
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0]["native_contract"]["label"], "AtipicialDollar::Transfer");
    assert!(value["warnings"].is_array());
    // Tokens JSON also surfaces script_hash for cross-correlation
    // with explorer URLs / info reports (parity with disasm and
    // decompile JSON which already exposed it).
    assert_eq!(
        value["script_hash_le"],
        Value::String("EAC5105B64136F5C84AE2C501E586A5AC67DE89D".into())
    );
    assert_eq!(
        value["script_hash_be"],
        Value::String("9DE87DC65A6A581E502CAE845C6F13645B10C5EA".into())
    );
    assert_schema(SchemaKind::Tokens, &value);
}

#[test]
fn tokens_command_handles_empty() {
    let dir = tempdir().expect("tempdir");
    let aef_path = dir.path().join("contract.aef");
    std::fs::write(&aef_path, build_aef_with_no_tokens()).unwrap();

    atipicial_decompiler_cmd()
        .arg("tokens")
        .arg(&aef_path)
        .assert()
        .success()
        .stdout(contains("no method tokens"));
}

#[test]
fn tokens_command_rejects_large_aef() {
    let dir = tempdir().expect("tempdir");
    let aef_path = dir.path().join("oversize.aef");
    write_oversize_aef(&aef_path);

    atipicial_decompiler_cmd()
        .arg("tokens")
        .arg(&aef_path)
        .assert()
        .failure()
        .stderr(contains("file size"))
        .stderr(contains("exceeds maximum"));
}
