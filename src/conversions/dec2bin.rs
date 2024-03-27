/// 十进制转二进制
///
/// # Arguments
///
/// * `base_num`: 十进制数
///
/// returns: String 二进制字符串
///
/// # Examples
///
/// ```
/// use algorithm::conversions::dec2bin;
/// assert_eq!(dec2bin(594), "1001010010");
/// ```
pub fn dec2bin(base_num: u128) -> String {
    if base_num == 0 { return "0".to_string(); }
    let mut dec = base_num;
    let mut bin = String::new();
    while dec != 0 {
        let bit = (dec % 2).to_string();
        bin.push_str(&bit);
        dec /= 2;
    };
    let bits = bin.chars();
    bits.rev().collect()
}

#[cfg(test)]
mod tests_dec2bin {
    use super::*;

    #[test]
    fn test_normal() {
        assert_eq!(dec2bin(594), "1001010010");
        assert_eq!(dec2bin(9), "1001");
        assert_eq!(dec2bin(0), "0");
    }
    #[test]
    fn test_big(){
        assert_eq!(
            dec2bin(u128::MAX),
            "1111111111111111111111111111111111111111111111111111111111111111\
            1111111111111111111111111111111111111111111111111111111111111111");
    }
}