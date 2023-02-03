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

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn base64_encode() {
        let str = "hello";
        let result = Base64::encode(str.as_bytes().to_vec());
        println!("{}", result);
    }

    #[test]
    fn base64_decode() {
        let str = "aGVsbG8=";
        let result = Base64::decode(str);


        match result {
            Ok(v) => {
                println!("{:?}", v.clone());
                println!("{:?}", String::from_utf8(v.clone()).unwrap().as_bytes());


                println!("{}", String::from_utf8(v.clone()).unwrap())
            }
            Err(_) => println!("failed!"),
        };
    }
}