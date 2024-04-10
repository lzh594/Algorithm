/// 计算组合数（二项式系数） (n choose k).
///
/// This function computes the comb_numial coefficient C(n, k) using BigInt
/// for arbitrary precision arithmetic.
///
/// Formula:
/// C(n, k) = n! / (k! * (n - k)!)
///
/// # Arguments
///
/// * `n` - 总数
/// * `k` - 选择数
///
/// # Returns
///
/// Returns the C(n, k) as a BigInt.
use num_bigint::BigInt;
use num_traits::FromPrimitive;

pub fn combinations(n: u64, k: u64) -> BigInt {
    let mut res = BigInt::from_u64(1).unwrap();
    for i in 0..k {
        res = (res * BigInt::from_u64(n - i).unwrap()) / BigInt::from_u64(i + 1).unwrap()
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb_num_5_2() {
        assert_eq!(combinations(5, 2), BigInt::from(10));
    }

    #[test]
    fn test_comb_num_10_5() {
        assert_eq!(combinations(10, 5), BigInt::from(252));
    }

    #[test]
    fn test_comb_num_0_0() {
        assert_eq!(combinations(0, 0), BigInt::from(1));
    }

    #[test]
    fn test_comb_num_large_n_small_k() {
        assert_eq!(combinations(1000, 2), BigInt::from(499500));
    }

    #[test]
    fn test_comb_num_random_1() {
        // Random test case 1
        assert_eq!(combinations(7, 4), BigInt::from(35));
    }

    #[test]
    fn test_comb_num_random_2() {
        // Random test case 2
        assert_eq!(combinations(12, 3), BigInt::from(220));
    }

    #[test]
    fn test_comb_num_random_3() {
        // Random test case 3
        assert_eq!(combinations(20, 10), BigInt::from(184_756));
    }
}