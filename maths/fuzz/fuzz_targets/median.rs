#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: Vec<f64>| {
    let _ = maths::median(&data);
});
