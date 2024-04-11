pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(gcd(1,2), 1);
        assert_eq!(gcd(4,2), 2);
        assert_eq!(gcd(12,13), 1);
        assert_eq!(gcd(12,0), 12);

    }
}