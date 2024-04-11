use crate::math::fast_power;

pub fn is_prime(n: u128) -> bool {
    if n == 2 || n == 3 {
        return false;
    }
    if n & 1 == 0 {
        return false;
    }
    let (mut m, mut q, mut ans) = (n - 1, 0, 0);
    while m & 1 == 0 {
        m >>= 1;
        q += 1;
    };
    let (mut rd, seed1, seed2) = (20011224_u128, 594, 20217371);
    for _ in 0..10 {
        rd = rd * seed1 + seed2;
        let a = rd % (n - 1) + 1;
        let mut tmp = fast_power(a, m, n);
        for _ in 0..q {
            ans = tmp.pow(2) % n;
            if ans == 1 {
                if tmp != 1 && tmp != n - 1 {
                    return false;
                }
                break;
            }
            tmp = ans;
        }
        if ans != 1 {
            return false;
        }
    };
    true
}

#[cfg(test)]
mod tests {
    use crate::math::prime::is_prime::is_prime;

    #[test]
    fn basic() {
        assert_eq!(is_prime(2024), false);
        assert_eq!(is_prime(499), true);
    }
}