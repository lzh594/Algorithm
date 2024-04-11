use crate::math::number_theory::invmod::invmod;

/// 中国剩余定理
///
/// # Arguments
///
/// * `r_lst`: 余数c列表
/// * `mod_lst`: 模数n列表
///
/// returns: i64 x = c (mod n)
///
/// # Examples
///
/// ```
///
/// ```
pub fn crt(r_lst: &[i64], mod_lst: &[i64]) -> Option<i64> {
    let m: i64 = mod_lst.iter().product();
    let mut sum: i64 = 0;
    for (&c_i, &n_i) in r_lst.iter().zip(mod_lst.iter()) {
        let m_i = m / n_i;
        let d_i = invmod(m_i, n_i);
        sum += (c_i * m_i * d_i?) % m;
    };
    Some((sum + m) % m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(crt(&[3, 5, 7], &[2, 3, 5]), Some(17));
        assert_eq!(crt(&[1, 4, 6], &[3, 5, 7]), Some(34));
        assert_eq!(crt(&[2, 5, 7], &[6, 9, 15]), None);
    }
}