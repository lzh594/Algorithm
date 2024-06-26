use std::mem::swap;


/// 扩展欧式算法
///
/// # Arguments
///
/// * `a`:
/// * `b`:
///
/// returns: (i64, i64, i64) = (g, u, v)
///
/// # Examples
///
/// ```
/// use algorithm::math::number_theory::egcd::egcd;
/// assert_eq!(egcd(50, 70), (10, 3, -2));
/// ```
pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut x = vec![a, 1, 0];
    let mut y = vec![b, 0, 1];
    while y[0] != 0 {
        let q = x[0] / y[0];
        for i in 0..3 {
            x[i] -= q * y[i];
        }
        swap(&mut x, &mut y);
    };
    (x[0], x[1], x[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(egcd(101, 13), (1, 4, -31));
        assert_eq!(egcd(123, 19), (1, -2, 13));
        assert_eq!(egcd(25, 36), (1, 13, -9));
        assert_eq!(egcd(69, 54), (3, -7, 9));
        assert_eq!(egcd(55, 79), (1, 23, -16));
        assert_eq!(egcd(33, 44), (11, -1, 1));
        assert_eq!(egcd(50, 70), (10, 3, -2));
    }
}