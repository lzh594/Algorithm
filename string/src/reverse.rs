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
///use string::reverse;
/// fn test_sentence() {
///    assert_eq!(reverse::reverse("step on no pets"), "step on no pets".to_string())
///}
/// ```
pub fn reverse(origin: &str) -> String {
    origin.chars().rev().collect()
}