/// 获取当前 host
pub fn get_host() -> Result<String, String> {
    match web_sys::window() {
        Some(window) => match window.location().host() {
            Ok(v) => Ok(v),
            Err(_) => Err(String::from("failed to resolve location.host"))
        }
        None => Err(String::from("unreachable window"))
    }
}