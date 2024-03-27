pub fn hamming_distance(str_a: &str, str_b: &str) -> usize {
    if str_a.len() != str_b.len() {
        panic!("Strings must have the same length!");
    }
    str_a.chars().zip(str_b.chars()).filter(|(a, b)| a != b).count()
}

#[cfg(test)]
mod test_hamming_distance {
    use super::*;
    // 声明宏
    macro_rules! test_normal {
        // 类似正则表达式
        ($($name:ident: $input:expr,)*) => {
        $(
            #[test]
            fn $name(){
                let (str_a,str_b,ans) = $input;
                assert_eq!(hamming_distance(str_a,str_b),ans);
                assert_eq!(hamming_distance(str_b,str_a),ans);
            }
        )*
        }
    }
    test_normal! {
        empty:("","",0),
        one:("1","1",0),
        same_strings: ("rust", "rust", 0),
        regular_input_0: ("karolin", "kathrin", 3),
        regular_input_1: ("kathrin", "kerstin", 4),
        regular_input_2: ("00000", "11111", 5),
        different_case: ("x", "X", 1),
    }
    #[test]
    #[should_panic]
    fn test_panic(){
        hamming_distance("","0");
    }
}