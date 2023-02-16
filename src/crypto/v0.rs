use wasm_bindgen::prelude::wasm_bindgen;
use crate::crypto::core::Core;

/// 可加密的原文最大长度为 `u32.MAX - 1`
#[wasm_bindgen]
pub struct V0 {}

#[allow(unused)]
#[wasm_bindgen]
impl V0 {
    /// 加密(若加密失败则返回空字符串)
    pub fn encode_base64(str: &str) -> String {
        Core::encode_base64("".to_string(), "".to_string(), str)
    }

    /// 解密(若加密失败则返回空字符串)
    pub fn decode_base64(str: &str) -> String {
        Core::decode_base64("".to_string(), "".to_string(), str)
    }
}

#[cfg(test)]
mod unit_test {
    use super::V0;

    #[test]
    fn serde() {
        let domain = "localhost:8000";

        let register_key = V0::encode_base64(domain);
        let unescaped_domain = V0::decode_base64(&register_key);

        println!("register_key: {}, unescaped_domain: {}", register_key, unescaped_domain);
    }
}