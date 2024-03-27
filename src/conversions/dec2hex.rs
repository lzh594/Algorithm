/// 十进制转十六进制
///
/// # Arguments
///
/// * `base_num`: 十进制数
///
/// returns: String 十六进制字符串
///
/// # Examples
///
/// ```
/// use algorithm::conversions::dec2hex;
/// assert_eq!(dec2hex(123456), "1E240");
/// ```
pub fn dec2hex(base_num: u128) -> String {
    if base_num == 0 { return "0".to_string(); }
    let mut dec = base_num;
    let mut hex = String::new();
    while dec != 0 {
        let remainder = dec % 16;
        let hex_char = if remainder < 10 { (remainder as u8 + b'0') as char } else { (remainder as u8 - 10 + b'A') as char };
        hex.insert(0, hex_char);
        dec /= 16;
    };
    hex
}

#[cfg(test)]
mod tests_dec2hex {
    use super::*;

    #[test]
    fn test_normal() {
        assert_eq!(dec2hex(0), "0");
        assert_eq!(dec2hex(9), "9");
        assert_eq!(dec2hex(12), "C");
        assert_eq!(dec2hex(255), "FF");
        assert_eq!(dec2hex(123456), "1E240");
    }

    #[test]
    fn test_big() {
        assert_eq!(dec2hex(u128::MAX), "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    }
}