use std::mem::swap;

// def Egcd(a: int, b: int) -> list:
// """
//     扩展欧拉算法:au+bv=g
//     :param a:
//     :param b:
//     :return: x = [g, u, v]
//     """
// if type_check(a, int) and type_check(b, int):
// pass
// x = [a, 1, 0]
// y = [b, 0, 1]
// while y[0] != 0:
// q = x[0] // y[0]
// for i in range(0, 3):
// x[i] = x[i] - q * y[i]
// tmp = x[:]
// x = y[:]
// y = tmp[:]
// return x
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