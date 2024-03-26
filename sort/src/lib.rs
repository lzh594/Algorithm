pub mod bubble;
pub mod quick;


#[cfg(test)]
mod tests_bubble {
    use super::*;

    #[test]
    fn test_empty() {
        let mut empty: Vec<String> = vec![];
        bubble::bubble_sort(&mut empty);
        assert_eq!(empty, Vec::<String>::new())
    }

    #[test]
    fn test_numbers() {
        let mut nums = vec![21, 43, 555, 12, 43, 432, 8765, 132, 6543, 11, 0, 24321];
        bubble::bubble_sort(&mut nums);
        assert_eq!(nums, vec![0, 11, 12, 21, 43, 43, 132, 432, 555, 6543, 8765, 24321]);
    }

    #[test]
    fn test_strings() {
        let mut strings = vec!["Bob".to_string(), "Alice".to_string(), "Eve".to_string(), "David".to_string()];
        bubble::bubble_sort(&mut strings);
        assert_eq!(strings, vec!["Alice".to_string(), "Bob".to_string(), "David".to_string(), "Eve".to_string()])
    }
}

#[cfg(test)]
mod tests_quick {
    use crate::quick;

    #[test]
    fn test_empty() {
        let mut empty: Vec<String> = vec![];
        quick::quick_sort(&mut empty);
        assert_eq!(empty, Vec::<String>::new())
    }

    #[test]
    fn test_numbers() {
        let mut nums = vec![21, 43, 555, 12, 43, 432, 8765, 132, 6543, 11, 0, 24321];
        quick::quick_sort(&mut nums);
        assert_eq!(nums, vec![0, 11, 12, 21, 43, 43, 132, 432, 555, 6543, 8765, 24321]);
    }

    #[test]
    fn test_strings() {
        let mut strings = vec!["Bob".to_string(), "Alice".to_string(), "Eve".to_string(), "David".to_string()];
        quick::quick_sort(&mut strings);
        assert_eq!(strings, vec!["Alice".to_string(), "Bob".to_string(), "David".to_string(), "Eve".to_string()])
    }
}