use aes::cipher::{
    block_padding::Pkcs7,
    BlockDecryptMut,
    BlockEncryptMut,
    KeyIvInit,
};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Window;
use crate::utils::{Base64, MD5};

enum SeedType { Key, Iv }

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128Enc>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128Dec>;

/// 可加密的原文最大长度为 `u32.MAX - 1`
#[wasm_bindgen]
pub struct V2 {
    // registered: bool,
}

#[allow(unused)]
#[wasm_bindgen]
impl V2 {
    // pub fn new() -> V2 {
    //     V2 { registered: false }
    // }
    //
    // pub fn register(&self, keys: Vec<String>) -> bool {
    //     for key in keys {}
    //     true
    // }

    pub fn prelude() -> String {
        match web_sys::window() {
            Some(window) => match window.location().host() {
                Ok(v) => v,
                Err(_) => "none".to_string()
            }
            None => "none".to_string()
        }
    }

    /// 计算密文所需的 buffer 大小
    // 大于原始字节长度的, 16的最小倍数 (注意: 16加密后为32, 类推)
    fn calc_container_size(plain: &str) -> usize {
        let byte_len = plain.as_bytes().len();
        (((byte_len as f32 / 16.0).floor() as usize) + 1) * 16
    }

    /// 构建md5原文
    fn generate_seed_str(seed_type: SeedType) -> String {
        format!("hello-world")
    }

    /// 生成 key
    fn generate_key() -> [u8; 16] { MD5::calc_buf(Self::generate_seed_str(SeedType::Key)) }

    /// 生成 iv
    fn generate_iv() -> [u8; 16] { MD5::calc_buf(Self::generate_seed_str(SeedType::Iv)) }

    /// 加密(若加密失败则返回空字符串)
    pub fn encode_base64(str: &str) -> String {
        let iv = Self::generate_iv();

        // 大于原始字节长度的, 16的最小倍数 (注意: 16加密后为32, 类推)
        let buf_size = Self::calc_container_size(str);
        let mut result_container = vec![0u8; buf_size];

        match Aes128CbcEnc::new(&Self::generate_key().into(), &iv.into())
            .encrypt_padded_b2b_mut::<Pkcs7>(str.as_bytes(), &mut result_container)
        {
            Ok(_) => Base64::encode(result_container),
            Err(_) => String::from("")
        }
    }

    /// 解密(若加密失败则返回空字符串)
    pub fn decode_base64(str: &str) -> String {
        let iv = Self::generate_iv();

        // 解密后的字节长度不会超过密文的字节长度
        let mut buf = Base64::decode(str).unwrap();

        match Aes128CbcDec::new(&Self::generate_key().into(), &iv.into())
            .decrypt_padded_mut::<Pkcs7>(&mut buf) {
            Ok(result_buffer) => match String::from_utf8(result_buffer.to_vec()) {
                Ok(result) => result,
                Err(_) => String::from("")
            }
            Err(_) => String::from("")
        }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn encode() {
        let str = "aaaaa";
        let result = V2::encode_base64(&str);
        println!("result: {}", result);
    }

    #[test]
    fn decode() {
        let str = "sWF+8MNMGiSBbjNy5Z9i2Q==";
        let result = V2::decode_base64(&str);
        println!("result: {}", result);
    }
}