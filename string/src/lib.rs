pub mod reverse;

#[cfg(test)]
mod tests_reverse {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(reverse::reverse("123456"), "654321".to_string())
    }

    #[test]
    fn test_sentence() {
        assert_eq!(reverse::reverse("step on no pets"), "step on no pets".to_string())
    }
}
