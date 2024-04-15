pub fn encode(message: &str, key: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(encode("", "test"), "");
    }

    #[test]
    fn encode_base() {
        assert_eq!(
            encode("LoremIpsumDolorSitAmet", "base"),
            "MojinIhwvmVsmojWjtSqft"
        );
    }

    #[test]
    fn encode_with_spaces() {
        assert_eq!(
            encode(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                "spaces",
            ),
            "Ddrgq ahhuo hgddr uml sbev, ggfheexwljr chahxsemfy tlkx."
        );
    }

    #[test]
    fn encode_unicode_and_numbers() {
        assert_eq!(
            encode("1 Lorem ⏳ ipsum dolor sit amet Ѡ", "unicode"),
            "1 Fbzga ⏳ ltmhu fcosl fqv opin Ѡ"
        );
    }

    #[test]
    fn encode_unicode_key() {
        assert_eq!(
            encode("Lorem ipsum dolor sit amet", "😉 key!"),
            "Vspoq gzwsw hmvsp cmr kqcd"
        );
    }

    #[test]
    fn encode_empty_key() {
        assert_eq!(encode("Lorem ipsum", ""), "Lorem ipsum");
    }
}