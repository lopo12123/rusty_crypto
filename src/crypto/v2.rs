use wasm_bindgen::prelude::wasm_bindgen;
use crate::crypto::core::Core;
use crate::glue::get_host;
use crate::utils::random_str;
use crate::V0;

/// 可加密的原文最大长度为 `u32.MAX - 1`
#[wasm_bindgen]
pub struct V2 {}

/// v2 是否已注册
static mut REGISTERED_V2: bool = false;

#[allow(unused)]
#[wasm_bindgen]
impl V2 {
    /// 解密得到 key
    fn unwrap_key(key: &str) -> String {
        V0::decode_base64(key)
    }

    /// 是否已注册
    pub fn is_registered() -> bool {
        unsafe {
            REGISTERED_V2
        }
    }

    /// 注册. 可重复调用(为减少消耗建议先调用 [Self::is_registered] 查看), 新的结果会覆盖旧结果 (若新key无效等效于调用 [Self::unregister]). 返回注册结果
    pub unsafe fn register(key: &str) -> bool {
        let key = Self::unwrap_key(key);

        // key 为空则直接返回 false
        if key.len() == 0 { return false; }

        match get_host() {
            Ok(host) => {
                let is_validated: bool = host == key;

                unsafe {
                    REGISTERED_V2 = is_validated;
                }

                is_validated
            }
            Err(_) => {
                unsafe {
                    REGISTERED_V2 = false;
                }

                false
            }
        }
    }

    /// 反注册
    pub unsafe fn unregister() {
        unsafe {
            REGISTERED_V2 = false;
        }
    }

    /// 加密(若未注册或加密失败则返回空字符串)
    pub fn encode_base64(str: &str) -> String {
        if !Self::is_registered() {
            return String::from("");
        }

        Core::encode_base64(random_str(1), random_str(1), str)
    }

    /// 解密(若未注册或加密失败则返回空字符串)
    pub fn decode_base64(str: &str) -> String {
        if !Self::is_registered() {
            return String::from("");
        }

        Core::decode_base64(random_str(1), random_str(1), str)
    }
}