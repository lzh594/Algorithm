use crate::math::number_theory::egcd::egcd;

/// 逆元
///
/// # Arguments
///
/// * `a`: 元素
/// * `n`: 模
///
/// returns: i64 逆元
///
/// # Examples
///
/// ```
/// use algorithm::math::number_theory::invmod::invmod;
/// assert_eq!(invmod(3, 7), Some(5));
/// ```
pub fn invmod(a: i64, n: i64) -> Option<i64> {
    assert!(n > 1, "n must be bigger than 1");
    let (g, x, _) = egcd(a, n);
    if g != 1 {
        return None;
    }
    Some((x+n) % n)
}

#[cfg(test)]
mod tests {
    use crate::math::number_theory::invmod::invmod;

    #[test]
    fn basic() {
        assert_eq!(invmod(3, 7), Some(5));
        assert_eq!(invmod(-3, 7), Some(2));
        // assert_eq!(invmod(-3, 0), Some(2));
        assert_eq!(invmod(0, 7), None);
    }
}