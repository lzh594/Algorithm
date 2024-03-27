/// 十六进制转十进制
///
/// # Arguments
///
/// * `hex_str`: 十六进制字符串
///
/// returns: Option<u128> 十进制数/None
///
/// # Examples
///
/// ```
/// use algorithm::conversions::hex2dec;
/// assert_eq!(hex2dec("1267A"), Some(75386));
/// ```
pub fn hex2dec(hex_str: &str) -> Option<u128> {
    if hex_str.len() > 32 {
        return None;
    }
    let mut dec: u128 = 0;
    let mut idx = 1;
    for hex in hex_str.chars().rev() {
        match hex {
            '0'..='9' => {
                if let Some(sum) = dec.checked_add(&idx * (hex as u8 - b'0') as u128) {
                    dec = sum;
                }
            }
            'A'..='F' => {
                if let Some(sum) = dec.checked_add(&idx * (hex as u8 + 10 - b'A') as u128) {
                    dec = sum;
                }
            }
            _ => return None
        }
        idx <<= 4;
    }
    Some(dec)
}

#[cfg(test)]
mod tests_hex2dec {
    use super::*;

    #[test]
    fn test_normal() {
        assert_eq!(hex2dec("45"), Some(69));
        assert_eq!(hex2dec("2B3"), Some(691));
        assert_eq!(hex2dec("4D2"), Some(1234));
        assert_eq!(hex2dec("1267A"), Some(75386));
    }

    #[test]
    fn test_big() {
        assert_eq!(hex2dec("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"), Some(u128::MAX));
    }

    #[test]
    fn test_overflow() {
        assert_eq!(hex2dec("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"), None);
    }
}