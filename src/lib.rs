mod utils;
mod serde;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use crate::serde::Serde;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn atob(s: &str) -> String;
    fn btoa(s: &str) -> String;
}

#[wasm_bindgen]
pub fn window_atob(s: &str) -> String {
    atob(s)
}
#[wasm_bindgen]
pub fn window_btoa(s: &str) -> String {
    btoa(s)
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hi there! -- from rust.");
}

#[wasm_bindgen]
pub fn encode_base64(s: &str) -> String {
    Serde::encode_base64(s)
}