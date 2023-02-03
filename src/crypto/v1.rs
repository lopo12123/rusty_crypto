use aes::cipher::{
    block_padding::Pkcs7,
    BlockDecryptMut,
    BlockEncryptMut,
    KeyIvInit,
};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::utils::{Base64, MD5};
use crate::glue::JsDate;

enum SeedType { Key, Iv }

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128Enc>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128Dec>;

/// 可加密的原文最大长度为 `u32.MAX - 1`
#[wasm_bindgen]
pub struct V1 {}

#[allow(unused)]
#[wasm_bindgen]
impl V1 {
    /// 计算密文所需的 buffer 大小
    // 大于原始字节长度的, 16的最小倍数 (注意: 16加密后为32, 类推)
    fn calc_container_size(plain: &str) -> usize {
        let byte_len = plain.as_bytes().len();
        (((byte_len as f32 / 16.0).floor() as usize) + 1) * 16
    }

    /// 构建md5原文
    fn generate_seed_str(seed_type: SeedType) -> String {
        format!(
            "catalyst_plus_{}{}{}{}{}{}",
            match seed_type {
                SeedType::Key => "key",
                SeedType::Iv => "iv"
            },
            JsDate::get_year_str(),
            JsDate::get_month_str(false),
            JsDate::get_day_str(false),
            JsDate::get_hour_str(false),
            JsDate::get_minute() / 10
        )
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
    use std::time::SystemTime;
    use super::*;

    #[test]
    fn encode_with_warmup() {
        fn run(i: u32) {
            let s = String::from("a".repeat(9000000));
            let t_start = SystemTime::now();
            let _result = V1::encode_base64(&s);
            println!("[{}] ok! elapsed: {}ms", i, t_start.elapsed().unwrap().as_millis());
        }

        for i in [0, 1, 2, 3] { run(i) }
    }

    #[test]
    fn decode_with_warmup() {
        let str = V1::encode_base64(&String::from("a".repeat(9000000)));

        let run = |i: u32| {
            let t_start = SystemTime::now();
            let result = V1::decode_base64(&str);
            println!("[{}]elapsed: {}ms", i, t_start.elapsed().unwrap().as_millis());

            let mut is_ok = true;
            for char in result.chars() {
                if char != 'a' {
                    is_ok = false;
                    break;
                }
            }
            println!("[{}] is ok: {}", i, is_ok);
        };

        for i in [0, 1, 2, 3] { run(i) }
    }

    #[test]
    fn encode() {
        let str = "aaaaa";
        let result = V1::encode_base64(&str);
        println!("result: {}", result);
    }

    #[test]
    fn decode() {
        let str = "sWF+8MNMGiSBbjNy5Z9i2Q==";
        let result = V1::decode_base64(&str);
        println!("result: {}", result);
    }


    #[test]
    fn misc() {
        for char in "abcdefg".to_string().chars() {
            println!("char: {}", char);
        }
    }
}