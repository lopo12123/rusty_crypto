extern crate wasm_bindgen;

mod utils;
mod glue;
mod crypto;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

pub use crypto::V1;

#[wasm_bindgen]
pub fn greet() {
    alert("Everything is rusted, and so is encryption.");
}
