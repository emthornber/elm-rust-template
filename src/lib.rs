mod utils;

use std::convert::TryInto;
use wasm_bindgen::prelude::*;

// // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// // allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn fibonacci(nth: i32) -> i32 {
    if nth < 0 {
        -1i32.pow((-nth + 1).try_into().unwrap()) * fibonacci(-nth)
    } else if nth == 0 {
        1
    } else if nth == 1 {
        1
    } else {
        fibonacci(nth - 2) + fibonacci(nth - 1)
    }
}
