use predicates::str::contains;
use tempfile::tempdir;

use crate::common::{build_aef_with_unknown_opcode, build_sample_aef, atipicial_decompiler_cmd};

#[test]
fn cfg_command_outputs_dot() {
    let dir = tempdir().expect("tempdir");
    let aef_path = dir.path().join("contract.aef");
    std::fs::write(&aef_path, build_sample_aef()).unwrap();

    atipicial_decompiler_cmd()
        .arg("cfg")
        .arg(&aef_path)
        .assert()
        .success()
        .stdout(contains("digraph CFG"))
        .stdout(contains("BB0"));
}

#[test]
fn cfg_can_fail_on_unknown_opcodes() {
    let dir = tempdir().expect("tempdir");
    let aef_path = dir.path().join("unknown.aef");
    std::fs::write(&aef_path, build_aef_with_unknown_opcode()).unwrap();

    // The default (non-failing) path still emits the graph but now surfaces
    // the disassembly warning on stderr, like `disasm` and `decompile`.
    atipicial_decompiler_cmd()
        .arg("cfg")
        .arg(&aef_path)
        .assert()
        .success()
        .stdout(contains("digraph CFG"))
        .stderr(contains("unknown opcode 0xFF"));

    atipicial_decompiler_cmd()
        .arg("cfg")
        .arg("--fail-on-unknown-opcodes")
        .arg(&aef_path)
        .assert()
        .failure()
        .stderr(contains("unknown opcode 0xFF"));
}
