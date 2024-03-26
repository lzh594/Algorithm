use std::cmp::Ordering;

/// 二分查找
///
/// # Arguments
///
/// * `item`: 查找目标
/// * `array`: 查找数组
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
pub fn binary_search<T: Ord>(target: &T, array: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = array.len();
    while left < right {
        let mid = left + (right - left) / 2;
        match target.cmp(&array[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
        }
    };
    None
}

/// 二分查找（递归）
///
/// # Arguments
///
/// * `target`: 查找目标
/// * `array`: 查找数组
/// * `low`: 查找下界
/// * `high`: 查找上界
///
/// returns: Option<usize>
///
/// # Examples
///
/// ```
///use search::binary;
/// fn test_numbers() {
///    let idx = binary::binary_search_rec(&5, &vec![1, 2, 3, 4, 5],&0,&5);
///    assert_eq!(idx, Some(4));
///}
/// ```
pub fn binary_search_rec<T: Ord>(target: &T, array: &[T], low: &usize, high: &usize) -> Option<usize> {
    if low >= high {
        return None;
    }
    let mid = low + (high - low) / 2;
    match target.cmp(&array[mid]) {
        Ordering::Less => binary_search_rec(target, array, low, &mid),
        Ordering::Equal => return Some(mid),
        Ordering::Greater => binary_search_rec(target, array, &mid, high),
    }
}