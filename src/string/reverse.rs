/// 字符串逆序倒转
///
/// # Arguments
///
/// * `origin`: 原始字符串
///
/// returns: String
///
/// # Examples
///
/// ```
/// use algorithm::string::reverse;
/// assert_eq!(reverse("step on no pets"), "step on no pets".to_string())
/// ```
pub fn reverse(origin: &str) -> String {
    origin.chars().rev().collect()
}

#[cfg(test)]
mod tests_reverse {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(reverse("123456"), "654321".to_string())
    }

    #[test]
    fn test_sentence() {
        assert_eq!(reverse("step on no pets"), "step on no pets".to_string())
    }
}