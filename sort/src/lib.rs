pub mod bubble;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty(){
        let mut empty:Vec<String> = vec![];
        bubble::bubble_sort(&mut empty);
        assert_eq!(empty, Vec::<String>::new())
    }

}
