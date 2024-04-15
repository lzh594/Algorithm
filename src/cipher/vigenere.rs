/// 维吉尼亚加密
///
/// # Arguments
///
/// * `message`: 明文
/// * `key`: 密钥
///
/// returns: String
///
/// # Examples
///
/// ```
/// use algorithm::cipher::vigenere_encode;
/// assert_eq!(vigenere_encode("Lorem ipsum dolor sit amet, consectetur adipiscing elit.","spaces"),          "Ddrgq ahhuo hgddr uml sbev, ggfheexwljr chahxsemfy tlkx.");
/// ```
pub fn vigenere_encode(message: &str, key: &str) -> String {
    let key: String = key.chars().filter(|&c| { c.is_ascii_alphabetic() }).collect();
    let key = key.to_lowercase();
    let key_len = key.len();
    if key_len == 0 {
        return String::from(message);
    }
    let mut index = 0;
    message.chars().map(|m| {
        if m.is_ascii_alphabetic() {
            let first = if m.is_ascii_lowercase() { b'a' } else { b'A' };
            let shift = key.as_bytes()[index % key_len] - b'a';
            index += 1;
            (first + (m as u8 + shift - first) % 26) as char
        } else {
            m
        }
    }).collect()
}

/// 维吉尼亚解密
///
/// # Arguments
///
/// * `cipher`: 密文
/// * `key`: 密钥
///
/// returns: String
///
/// # Examples
///
/// ```
/// use algorithm::cipher::vigenere::vigenere_decode;
/// assert_eq!(vigenere_decode("Vspoq gzwsw hmvsp cmr kqcd", "😉 key!"),"Lorem ipsum dolor sit amet");
/// ```
pub fn vigenere_decode(cipher: &str, key: &str) -> String {
    let key: String = key.chars().filter(|&c| { c.is_ascii_alphabetic() }).collect();
    let key = key.to_lowercase();
    let key_len = key.len();
    if key_len == 0 {
        return String::from(cipher);
    }
    let mut index = 0;
    cipher.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shift = key.as_bytes()[index % key_len] - b'a';
            index += 1;
            (first + (c as u8 + 26 - shift - first) % 26) as char
        } else {
            c
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(vigenere_encode("", "test"), "");
        assert_eq!(
            vigenere_encode("LoremIpsumDolorSitAmet", "base"),
            "MojinIhwvmVsmojWjtSqft");
        assert_eq!(
            vigenere_encode(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                "spaces",),
            "Ddrgq ahhuo hgddr uml sbev, ggfheexwljr chahxsemfy tlkx.");
        assert_eq!(
            vigenere_encode("1 Lorem ⏳ ipsum dolor sit amet Ѡ", "unicode"),
            "1 Fbzga ⏳ ltmhu fcosl fqv opin Ѡ");
        assert_eq!(
            vigenere_encode("Lorem ipsum dolor sit amet", "😉 key!"),
            "Vspoq gzwsw hmvsp cmr kqcd");
        assert_eq!(vigenere_encode("Lorem ipsum", ""), "Lorem ipsum");
    }
    #[test]
    fn test_decode(){
        assert_eq!(vigenere_decode("", "test"), "");
        assert_eq!(
            vigenere_decode("MojinIhwvmVsmojWjtSqft", "base"),
            "LoremIpsumDolorSitAmet");
        assert_eq!(
            vigenere_decode(
                "Ddrgq ahhuo hgddr uml sbev, ggfheexwljr chahxsemfy tlkx.",
                "spaces",),
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
        assert_eq!(
            vigenere_decode("1 Fbzga ⏳ ltmhu fcosl fqv opin Ѡ", "unicode"),
            "1 Lorem ⏳ ipsum dolor sit amet Ѡ");
        assert_eq!(
            vigenere_decode("Vspoq gzwsw hmvsp cmr kqcd", "😉 key!"),
            "Lorem ipsum dolor sit amet");
        assert_eq!(vigenere_decode("Lorem ipsum", ""), "Lorem ipsum");
    }
}