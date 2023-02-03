use js_sys::{Date};

fn get_js_date() -> Date { Date::new_0() }

/// js `Date` 对象的一些操作的封装
pub struct JsDate {}

#[allow(unused)]
impl JsDate {
    ///  获取 年
    pub fn get_year() -> u32 { get_js_date().get_full_year() }

    ///  获取 年
    pub fn get_year_str() -> String { get_js_date().get_full_year().to_string() }

    ///  获取 年 UTC
    pub fn get_year_utc() -> u32 { get_js_date().get_utc_full_year() }

    ///  获取 年 UTC
    pub fn get_year_utc_str() -> String { get_js_date().get_utc_full_year().to_string() }

    ///  获取 月 (1 - 12)
    pub fn get_month() -> u32 { get_js_date().get_month() + 1 }

    ///  获取 月 (1 - 12) (`pad` 是否补齐)
    pub fn get_month_str(pad: bool) -> String {
        let _v: u32 = get_js_date().get_month() + 1;

        if pad && (_v < 10) { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 月 UTC (1 - 12)
    pub fn get_month_utc() -> u32 { get_js_date().get_utc_month() + 1 }

    ///  获取 月 UTC (1 - 12) (`pad` 是否补齐)
    pub fn get_month_utc_str(pad: bool) -> String {
        let _v: u32 = get_js_date().get_utc_month() + 1;

        if pad && (_v < 10) { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 日
    pub fn get_day() -> u32 { get_js_date().get_date() }

    ///  获取 日 (`pad` 是否补齐)
    pub fn get_day_str(pad: bool) -> String {
        let _v: u32 = get_js_date().get_date();

        if pad && (_v < 10) { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 日
    pub fn get_day_utc() -> u32 { get_js_date().get_utc_date() }

    ///  获取 日 (`pad` 是否补齐)
    pub fn get_day_utc_str(pad: bool) -> String {
        let _v: u32 = get_js_date().get_utc_date();

        if pad && (_v < 10) { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 时
    pub fn get_hour() -> u32 { get_js_date().get_hours() }

    ///  获取 时 (`pad` 是否补齐)
    pub fn get_hour_str(pad: bool) -> String {
        let _v: u32 = get_js_date().get_hours();

        if pad && (_v < 10) { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 时 UTC
    pub fn get_hour_utc() -> u32 { get_js_date().get_utc_hours() }

    ///  获取 时 UTC (`pad` 是否补齐)
    pub fn get_hour_utc_str(pad: bool) -> String {
        let _v: u32 = get_js_date().get_utc_hours();

        if pad && (_v < 10) { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 分
    pub fn get_minute() -> u32 { get_js_date().get_minutes() }

    ///  获取 分 (`pad` 是否补齐)
    pub fn get_minute_str(pad: bool) -> String {
        let _v: u32 = get_js_date().get_minutes();

        if pad && (_v < 10) { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 分 UTC
    pub fn get_minute_utc() -> u32 { get_js_date().get_utc_minutes() }

    ///  获取 分 UTC (`pad` 是否补齐)
    pub fn get_minute_utc_str(pad: bool) -> String {
        let _v: u32 = get_js_date().get_utc_minutes();

        if pad && (_v < 10) { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 秒
    pub fn get_second() -> u32 { get_js_date().get_seconds() }

    ///  获取 秒 (`pad` 是否补齐)
    pub fn get_second_str(pad: bool) -> String {
        let _v: u32 = get_js_date().get_seconds();

        if pad && (_v < 10) { format!("0{}", _v) } else { _v.to_string() }
    }

    ///  获取 秒 UTC
    pub fn get_second_utc() -> u32 { get_js_date().get_utc_seconds() }

    ///  获取 秒 UTC (`pad` 是否补齐)
    pub fn get_second_utc_str(pad: bool) -> String {
        let _v: u32 = get_js_date().get_utc_seconds();

        if pad && (_v < 10) { format!("0{}", _v) } else { _v.to_string() }
    }

    /// 获取 时间戳
    pub fn get_time() -> f64 {
        get_js_date().get_time()
    }
}