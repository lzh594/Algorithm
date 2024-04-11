use num_traits::Pow;
use rand::Rng;
use crate::math::prime::is_prime::is_prime;

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