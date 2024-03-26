pub mod binary;

#[cfg(test)]
mod tests_binary {
    use super::*;

    #[test]
    fn test_empty() {
        let idx = binary::binary_search(&"594", &vec![]);
        assert_eq!(idx, None);
    }

    #[test]
    fn test_one() {
        let idx = binary::binary_search(&"594", &vec!["594"]);
        assert_eq!(idx, Some(0));
    }

    #[test]
    fn test_strings() {
        let idx = binary::binary_search(&"594", &vec!["11", "book", "12dzf", "594", "lzh"]);
        assert_eq!(idx, Some(3));
    }

    #[test]
    fn test_numbers() {
        let idx = binary::binary_search(&3, &vec![1, 2, 3, 4, 5]);
        assert_eq!(idx, Some(2));
    }
}
#[cfg(test)]
mod tests_binary_rec{
    use super::*;

    #[test]
    fn test_empty() {
        let idx = binary::binary_search_rec(&"594", &vec![],&0,&0);
        assert_eq!(idx, None);
    }

    #[test]
    fn test_one() {
        let idx = binary::binary_search_rec(&"594", &vec!["594"],&0,&1);
        assert_eq!(idx, Some(0));
    }

    #[test]
    fn test_strings() {
        let idx = binary::binary_search_rec(&"594", &vec!["11", "book", "12dzf", "594", "lzh"],&0,&5);
        assert_eq!(idx, Some(3));
    }

    #[test]
    fn test_numbers() {
        let idx = binary::binary_search_rec(&5, &vec![1, 2, 3, 4, 5],&0,&5);
        assert_eq!(idx, Some(4));
    }
}