use std::cmp::Ordering;

///
///
/// # Arguments
///
/// * `item`:
/// * `array`:
///
/// returns: Option<usize>
///
/// # Examples
///
/// ```
///use search::binary;
/// fn test_strings() {
///    let idx = binary::binary_search(&"594", &vec!["11", "book", "12dzf", "594", "lzh"]);
///    assert_eq!(idx, Some(3));
///}
/// ```
pub fn binary_search<T: Ord>(item: &T, array: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len();
    while left < right {
        let mid = left + (right - left) / 2;
        match item.cmp(&array[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
        }
    };
    None
}