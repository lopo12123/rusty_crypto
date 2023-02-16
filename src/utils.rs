use base64::{decode, DecodeError, encode};
use js_sys::Math::random;
use md5::Digest;

pub struct Base64 {}

#[allow(unused)]
impl Base64 {
    pub fn decode(s: &str) -> Result<Vec<u8>, DecodeError> {
        decode(s)
    }

    pub fn encode(bytes: Vec<u8>) -> String {
        encode(bytes)
    }
}

pub struct MD5 {}

#[allow(unused)]
impl MD5 {
    pub fn calc(str: String) -> Digest { md5::compute(str) }

    pub fn calc_buf(str: String) -> [u8; 16] { md5::compute(str).0 }
}

/// 构建指定长度(1-20)的随机字符串 (a-z), 超过长度范围则返回 "a", 失败则返回以 "a" 填充的指定长度字符串
pub fn random_str(len: usize) -> String {
    if len <= 0 || len > 20 {
        return String::from("a");
    }

    let mut char_codes: Vec<u8> = vec![];
    for _ in 0..len {
        char_codes.push(97 + (random() * 26_f64).floor() as u8)
    }

    match String::from_utf8(char_codes) {
        Ok(str) => str,
        Err(_) => "a".repeat(len)
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn md5() {
        println!("{:?}", MD5::calc_buf("123".to_string()));
    }
}