use base64::{decode, DecodeError, encode};

pub fn base64_str2bytes(s: &str) -> Result<Vec<u8>, DecodeError> {
    decode(s)
}

pub fn base64_bytes2str(bytes: Vec<u8>) -> String {
    encode(bytes)
}