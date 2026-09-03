use predicates::str::contains;
use serde_json::Value;
use tempfile::tempdir;

use crate::common::{
    assert_schema, build_aef_with_unknown_opcode, build_sample_aef, atipicial_decompiler_cmd, SchemaKind,
};

#[test]
fn disasm_command_outputs_instructions() {
    let dir = tempdir().expect("tempdir");
    let aef_path = dir.path().join("contract.aef");
    std::fs::write(&aef_path, build_sample_aef()).unwrap();

    let output = atipicial_decompiler_cmd()
        .arg("disasm")
        .arg(&aef_path)
        .output()
        .expect("disasm output");
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("0000: PUSH0"));

    let json_output = atipicial_decompiler_cmd()
        .arg("disasm")
        .arg("--format")
        .arg("json")
        .arg(&aef_path)
        .output()
        .expect("json disasm");
    assert!(json_output.status.success());
    let value: Value = serde_json::from_slice(&json_output.stdout).expect("json parse");
    let instructions = value["instructions"].as_array().expect("array");
    assert_eq!(instructions[0]["opcode"], "PUSH0");
    assert_eq!(instructions[0]["offset"], 0);
    assert_eq!(instructions[0]["operand_kind"], Value::String("I32".into()));
    assert_eq!(
        instructions[0]["operand_value"]["type"],
        Value::String("I32".into())
    );
    assert_eq!(instructions[0]["operand_value"]["value"], Value::from(0));
    assert_eq!(instructions[1]["operand_kind"], Value::String("I32".into()));
    assert_eq!(instructions[1]["operand_value"]["value"], Value::from(1));
    assert!(value["warnings"].is_array());
    // Disasm JSON now surfaces script_hash so callers can correlate
    // an instruction stream against an `info` / `decompile` report or
    // explorer URL without re-parsing the AEF themselves.
    assert_eq!(
        value["script_hash_le"],
        Value::String("EAC5105B64136F5C84AE2C501E586A5AC67DE89D".into())
    );
    assert_eq!(
        value["script_hash_be"],
        Value::String("9DE87DC65A6A581E502CAE845C6F13645B10C5EA".into())
    );
    assert_schema(SchemaKind::Disasm, &value);
}

#[test]
fn disasm_can_fail_on_unknown_opcodes() {
    let dir = tempdir().expect("tempdir");
    let aef_path = dir.path().join("unknown.aef");
    std::fs::write(&aef_path, build_aef_with_unknown_opcode()).unwrap();

    atipicial_decompiler_cmd()
        .arg("disasm")
        .arg(&aef_path)
        .assert()
        .success()
        .stdout(contains("UNKNOWN_0xFF"))
        .stdout(contains("0001: RET"));

    atipicial_decompiler_cmd()
        .arg("disasm")
        .arg("--fail-on-unknown-opcodes")
        .arg(&aef_path)
        .assert()
        .failure()
        .stderr(contains("unknown opcode 0xFF"));
}
