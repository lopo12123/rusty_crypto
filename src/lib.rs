extern crate wasm_bindgen;

mod utils;
mod glue;
mod crypto;

use wasm_bindgen::prelude::*;
use crypto::V1;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Everything is rusted, and so is encryption.");
}

#[wasm_bindgen]
pub fn encode_base64(s: &str) -> String {
    V1::encode_base64(s)
}
