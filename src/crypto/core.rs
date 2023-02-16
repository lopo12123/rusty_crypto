use aes::cipher::{
    block_padding::Pkcs7,
    BlockDecryptMut,
    BlockEncryptMut,
    KeyIvInit,
};
use crate::utils::{Base64, MD5};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128Enc>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128Dec>;

/// 可加密的原文最大长度为 `u32.MAX - 1`
pub struct Core {}

#[allow(unused)]
impl Core {
    /// 计算密文所需的 buffer 大小
    // 大于原始字节长度的, 16的最小倍数 (注意: 16加密后为32, 类推)
    fn calc_container_size(plain: &str) -> usize {
        let byte_len = plain.as_bytes().len();
        (((byte_len as f32 / 16.0).floor() as usize) + 1) * 16
    }

    /// 加密(若加密失败则返回空字符串)
    pub fn encode_base64(plain_key: String, plain_iv: String, str: &str) -> String {
        let iv = MD5::calc_buf(plain_iv);
        let key = MD5::calc_buf(plain_key);

        // 大于原始字节长度的, 16的最小倍数 (注意: 16加密后为32, 类推)
        let buf_size = Self::calc_container_size(str);
        let mut result_container = vec![0u8; buf_size];

        match Aes128CbcEnc::new(&key.into(), &iv.into())
            .encrypt_padded_b2b_mut::<Pkcs7>(str.as_bytes(), &mut result_container)
        {
            Ok(_) => Base64::encode(result_container),
            Err(_) => String::from("")
        }
    }

    /// 解密(若加密失败则返回空字符串)
    pub fn decode_base64(plain_key: String, plain_iv: String, str: &str) -> String {
        let iv = MD5::calc_buf(plain_iv);
        let key = MD5::calc_buf(plain_key);

        // 解密后的字节长度不会超过密文的字节长度
        let mut buf = vec![];

        match Base64::decode(str) {
            Ok(_buf) => buf = _buf,
            Err(_) => {
                return String::from("");
            }
        }

        match Aes128CbcDec::new(&key.into(), &iv.into())
            .decrypt_padded_mut::<Pkcs7>(&mut buf) {
            Ok(result_buffer) => match String::from_utf8(result_buffer.to_vec()) {
                Ok(result) => result,
                Err(_) => String::from("")
            }
            Err(_) => String::from("")
        }
    }
}