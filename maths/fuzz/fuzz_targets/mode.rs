#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: Vec<i32>| {
    let _ = maths::mode(&data);
});
