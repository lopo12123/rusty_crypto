use base64::{decode, DecodeError, encode};
use time::Month;

/// `1-12`
///
/// ```rust
/// # use time::Month;
/// assert_eq!(month_to_index(Month::January), 1);
/// assert_eq!(month_to_index(Month::August), 8);
/// ```
pub fn month_to_index(month: Month) -> u8 {
    match month {
        Month::January => 1,
        Month::February => 2,
        Month::March => 3,
        Month::April => 4,
        Month::May => 5,
        Month::June => 6,
        Month::July => 7,
        Month::August => 8,
        Month::September => 9,
        Month::October => 10,
        Month::November => 11,
        Month::December => 12,
    }
}

pub fn base64_str2bytes(s: &str) -> Result<Vec<u8>, DecodeError> {
    decode(s)
}

pub fn base64_bytes2str(bytes: Vec<u8>) -> String {
    encode(bytes)
}


#[cfg(test)]
mod unit_test {
    use super::*;
}