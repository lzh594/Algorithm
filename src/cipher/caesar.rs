/// 凯撒加密
///
/// # Arguments
///
/// * `message`: 明文
/// * `shift`: 密钥
///
/// returns: String
///
/// # Examples
///
/// ```
/// use algorithm::cipher::encode;
/// assert_eq!(encode("attack at dawn 攻", 5), "fyyfhp fy ifbs 攻");
/// ```
pub fn encode(message: &str, shift: u8) -> String {
    message.chars()
        .map(|m| {
            if m.is_ascii_alphabetic() {
                let first = if m.is_ascii_lowercase() { b'a' } else { b'A' };
                (first + (m as u8 + shift - first) % 26) as char
            } else {
                m
            }
        }).collect()
}

/// 凯撒解密
///
/// # Arguments
///
/// * `cipher`: 密文
/// * `shift`: 密钥
///
/// returns: String
///
/// # Examples
///
/// ```
/// use algorithm::cipher::decode;
/// assert_eq!(decode("fyyfhp fy ifbs 攻", 5), "attack at dawn 攻");
/// ```
pub fn decode(cipher: &str, shift: u8) -> String {
    cipher.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                // 注意溢出
                (first + (c as u8 + 26 - shift - first) % 26) as char
            } else {
                c
            }
        }).collect()
}

#[cfg(test)]
mod tests {
    use crate::cipher::caesar::{decode, encode};

    #[test]
    fn test_encode() {
        assert_eq!(encode("", 13), "");
        assert_eq!(encode("rust", 13), "ehfg");
        assert_eq!(encode("attack at dawn 攻", 5), "fyyfhp fy ifbs 攻");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("", 13), "");
        assert_eq!(decode("ehfg", 13), "rust");
        assert_eq!(decode("fyyfhp fy ifbs 攻", 5), "attack at dawn 攻");
    }
}