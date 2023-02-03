use base64::{decode, DecodeError, encode};

pub struct Base64 {}

impl Base64 {
    pub fn decode(s: &str) -> Result<Vec<u8>, DecodeError> {
        decode(s)
    }

    pub fn encode(bytes: Vec<u8>) -> String {
        encode(bytes)
    }
}