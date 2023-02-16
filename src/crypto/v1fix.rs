use wasm_bindgen::prelude::wasm_bindgen;
use crate::crypto::core::Core;
use crate::glue::JsDate;

enum SeedType { Key, Iv }

/// 可加密的原文最大长度为 `u32.MAX - 1`
#[wasm_bindgen]
pub struct V1Fix {}

#[allow(unused)]
#[wasm_bindgen]
impl V1Fix {
    /// 构建md5原文
    fn generate_seed_str(seed_type: SeedType) -> String {
        format!(
            "catalyst_plus_{}{}{}{}{}{}",
            match seed_type {
                SeedType::Key => "key",
                SeedType::Iv => "iv"
            },
            JsDate::get_year_utc_str(),
            JsDate::get_month_utc_str(false),
            JsDate::get_day_utc_str(false),
            JsDate::get_hour_utc_str(false),
            JsDate::get_minute_utc() / 10
        )
    }

    /// 加密(若加密失败则返回空字符串)
    pub fn encode_base64(str: &str) -> String {
        Core::encode_base64(
            Self::generate_seed_str(SeedType::Key),
            Self::generate_seed_str(SeedType::Iv),
            str,
        )
    }

    /// 解密(若加密失败则返回空字符串)
    pub fn decode_base64(str: &str) -> String {
        Core::decode_base64(
            Self::generate_seed_str(SeedType::Key),
            Self::generate_seed_str(SeedType::Iv),
            str,
        )
    }
}