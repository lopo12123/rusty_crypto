use aes::cipher::{
    block_padding::Pkcs7,
    BlockDecryptMut,
    BlockEncryptMut,
    KeyIvInit,
};
use crate::utils::{base64_bytes2str};

enum SeedType { Key, Iv }

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128Enc>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128Dec>;

/// 可加密的原文最大长度为 `u32.MAX - 1`
pub struct Serde {}

#[allow(unused)]
impl Serde {
    /// 计算字符串的 md5 值
    fn calc_md5(str: String) -> [u8; 16] {
        md5::compute(str).0
    }

    /// 计算密文所需的 buffer 大小
    // 大于原始字节长度的, 16的最小倍数 (注意: 16加密后为32, 类推)
    fn calc_buf_size(plain: &str) -> usize {
        let byte_len = plain.as_bytes().len();
        (((byte_len as f32 / 16.0).floor() as usize) + 1) * 16
    }

    /// 构建md5原文
    fn generate_seed_str(seed_type: SeedType) -> String {
        // let date = OffsetDateTime::now_utc();
        //
        // format!("catalyst_plus_{}{}{}{}{}{}", match seed_type {
        //     SeedType::Key => "key",
        //     SeedType::Iv => "iv"
        // }, date.year(), month_to_index(date.month()), date.day(), date.hour() + 8, date.minute() / 10)
        String::from("1234567890")
    }

    /// 生成 key
    fn generate_key() -> [u8; 16] {
        Self::calc_md5(Self::generate_seed_str(SeedType::Key))
    }

    /// 生成 iv
    fn generate_iv() -> [u8; 16] {
        Self::calc_md5(Self::generate_seed_str(SeedType::Iv))
    }

    /// 加密(若加密失败则返回空字符串) - 已测试
    pub fn encode_base64(str: &str) -> String {
        let iv = Self::generate_iv();

        // 大于原始长度的, 16的最小倍数 (注意: 16加密后为32, 类推)
        let buf_size = Self::calc_buf_size(str);
        let mut result_container = vec![0u8; buf_size];

        match Aes128CbcEnc::new(&Self::generate_key().into(), &iv.into())
            .encrypt_padded_b2b_mut::<Pkcs7>(str.as_bytes(), &mut result_container)
        {
            Ok(_) => base64_bytes2str(result_container),
            Err(_) => String::from("")
        }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use crate::utils::{base64_str2bytes, base64_bytes2str};

    #[test]
    fn encode() {
        let str = "hello";
        let encoded = Serde::encode_base64(str);
        println!("encoded: {:?}", encoded);
    }

    #[test]
    fn branch_encode() {
        for len in [90000000] {
            let s = String::from("a".repeat(len));
            let result = Serde::encode_base64(&s);
            // println!("original: {}\nencoded: {}\n\n", s, result);
            if result.len() == 0 {
                println!("error at {}", len);
            }
        }
        println!("ok!");
    }

    #[test]
    fn vec_size() {
        // let mut v = vec![1u8, 2, 3, 4];
        // let mut v: Vec<u8> = Vec::with_capacity(4);
        let n = 10;
        let mut v = vec![0; n];
        let b = &mut v[..];

        println!("{}", b.len());
    }

    #[test]
    fn vec_to_str() {
        let bytes: Vec<u8> = vec![247, 212, 95, 28, 55, 20, 23, 202, 4, 33, 145, 249, 88, 149, 158, 85];

        let s = base64_bytes2str(bytes);
        println!("s: {}", s);
    }
}