#![no_main]

use libfuzzer_sys::fuzz_target;
use atipicial_decompiler::Decompiler;

fuzz_target!(|data: &[u8]| {
    // Exercise the full decompilation pipeline; we only care about panics.
    let _ = Decompiler::new().decompile_bytes(data);
});
