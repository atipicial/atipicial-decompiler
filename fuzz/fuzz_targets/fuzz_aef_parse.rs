#![no_main]

use libfuzzer_sys::fuzz_target;
use atipicial_decompiler::AefParser;

fuzz_target!(|data: &[u8]| {
    let parser = AefParser::new();
    // We don't care about the result, only that it doesn't panic
    let _ = parser.parse(data);
});
