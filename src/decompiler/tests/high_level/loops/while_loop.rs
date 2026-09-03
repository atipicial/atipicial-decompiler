use super::*;

#[test]
fn high_level_lifts_simple_while_loop() {
    // Script: PUSH1, JMPIFNOT +5 (to RET), NOP, JMP -4 (to PUSH1), RET
    let script = [0x11, 0x26, 0x05, 0x21, 0x22, 0xFC, 0x40];
    let aef_bytes = build_aef(&script);
    let decompilation = Decompiler::new()
        .decompile_bytes(&aef_bytes)
        .expect("decompile succeeds");

    let high_level = decompilation
        .high_level
        .as_deref()
        .expect("high-level output");
    assert!(
        high_level.contains("while t0 {"),
        "missing while block: {high_level}"
    );
    assert!(
        !high_level.contains("jump ->"),
        "loop back-edge should be lifted: {high_level}"
    );
}
