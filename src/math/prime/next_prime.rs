use crate::math::prime::is_prime::is_prime;

/// 获取最近的下一个素数
///
/// # Arguments
///
/// * `n`: 当前整数
///
/// returns: u128
///
/// # Examples
///
/// ```
/// use algorithm::math::prime::next_prime::next_prime;
/// assert_eq!(next_prime(1120),1123);
/// ```
pub fn next_prime(n: u128) -> u128 {
    let mut next_n = (n.clone() + 1) | 1;
    while !is_prime(next_n) {
        next_n += 2;
    }
    next_n
}

#[cfg(test)]
mod test{
    use crate::math::prime::next_prime::next_prime;

    #[test]
    fn basic(){
        assert_eq!(next_prime(3),5);
        assert_eq!(next_prime(11),13);
        assert_eq!(next_prime(1120),1123);
    }
}