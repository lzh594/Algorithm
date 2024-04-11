pub fn fast_power(mut base: usize, mut power: usize, modulus: usize) -> usize {
    let mut res: usize = 1;
    if modulus == 0 {
        while power > 0 {
            if power & 1 == 1 {
                res *= base;
            }
            power >>= 1;
            base *= base;
        }
    } else {
        while power > 0 {
            if power & 1 == 1 {
                res = (res * base) % modulus;
            }
            power >>= 1;
            base = (base*base) % modulus;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Need to be careful about large exponents. It is easy to hit overflows.
        assert_eq!(fast_power(2, 3, 0), 8);
        assert_eq!(fast_power(4, 12, 0), 16777216);
        assert_eq!(fast_power(6, 12, 0), 2176782336);
        assert_eq!(fast_power(10, 4, 0), 10000);
        assert_eq!(fast_power(20, 3, 0), 8000);
        assert_eq!(fast_power(3, 21, 0), 10460353203);
    }
    #[test]
    fn test_modulus() {
        assert_eq!(fast_power(2, 3, 2), 0);
        assert_eq!(fast_power(2, 10, 10), 4);
        assert_eq!(fast_power(3, 21, 7), 6);
        assert_eq!(fast_power(2, 100, 1000000007), 976371285);
    }
}