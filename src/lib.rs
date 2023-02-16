extern crate wasm_bindgen;

mod utils;
mod glue;
mod crypto;

// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

pub use crypto::V0;
pub use crypto::V1;
pub use crypto::V1Fix;
pub use crypto::V2;