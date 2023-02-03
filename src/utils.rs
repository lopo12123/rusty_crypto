use base64::{decode, DecodeError, encode};
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