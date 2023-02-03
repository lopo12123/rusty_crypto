mod utils;
mod serde;
mod glue;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use crate::serde::Serde;

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
    Serde::encode_base64(s)
}
