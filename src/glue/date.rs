use js_sys::{Date};

fn get_js_date() -> Date { Date::new_0() }

/// js `Date` 对象的一些操作的封装
pub struct JsDate {}

#[allow(unused)]
impl JsDate {
    ///  获取 年
    pub fn get_year() -> String {
        get_js_date().get_utc_full_year().to_string()
    }

    ///  获取 月
    /// `pad` 补齐长度(补齐方式: 头部用0填充)
    pub fn get_month(pad: bool) -> String {
        let _v: u32 = get_js_date().get_utc_month();

        if pad && _v < 10 { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 日
    /// `pad` 补齐长度(补齐方式: 头部用0填充)
    pub fn get_day(pad: bool) -> String {
        let _v: u32 = get_js_date().get_utc_date();

        if pad && _v < 10 { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 时
    /// `pad` 补齐长度(补齐方式: 头部用0填充)
    pub fn get_hour(pad: bool) -> String {
        let _v: u32 = get_js_date().get_utc_hours();

        if pad && _v < 10 { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 分
    /// `pad` 补齐长度(补齐方式: 头部用0填充)
    pub fn get_minute(pad: bool) -> String {
        let _v: u32 = get_js_date().get_utc_minutes();

        if pad && _v < 10 { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 秒
    /// `pad` 补齐长度(补齐方式: 头部用0填充)
    pub fn get_second(pad: bool) -> String {
        let _v: u32 = get_js_date().get_utc_seconds();

        if pad && _v < 10 { format!("0{}", _v) } else { _v.to_string() }
    }

    /// 获取 时间戳
    pub fn get_time() -> f64 {
        get_js_date().get_time()
    }
}