use num_traits::Pow;
use rand::Rng;
use crate::math::prime::is_prime::is_prime;

/// 获取一定二进制长度的十进制素数
///
/// # Arguments
///
/// * `len_bit`: 二进制长度
///
/// returns: u128
///
/// # Examples
///
/// ```
/// use algorithm::math::prime::{get_prime, is_prime};
/// assert_eq!(is_prime(get_prime(30)), true);
/// ```
pub fn get_prime(len_bit: usize) -> u128 {
    let mut prm = ((rand::thread_rng().gen_range(2.pow(len_bit - 1)..2.pow(len_bit)) + 1) | 1) as u128;
    while !is_prime(prm.try_into().unwrap()) {
        prm += 2;
    }
    prm
}

#[cfg(test)]
mod tests {
    use crate::math::prime::get_prime::get_prime;
    use crate::math::prime::is_prime::is_prime;

    #[test]
    fn basic() {
        assert_eq!(is_prime(get_prime(10)), true);
        assert_eq!(is_prime(get_prime(20)), true);
        assert_eq!(is_prime(get_prime(30)), true);
    }
}