/// 快速排序
///
/// # Arguments
///
/// * `array`: 待排序数组
///
/// returns: ()
///
/// # Examples
///
/// ```
/// use algorithm::sort::quick_sort;
/// let mut nums = vec![21, 43, 555, 12, 43, 432, 8765, 132, 6543, 11, 0, 24321];
/// quick_sort(&mut nums);
/// assert_eq!(nums, vec![0, 11, 12, 21, 43, 43, 132, 432, 555, 6543, 8765, 24321]);
/// ```
pub fn quick_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len > 1 {
        quick_sort_range(array, 0, len - 1)
    }
}

fn quick_sort_range<T: PartialOrd>(array: &mut [T], left: usize, right: usize) {
    if left < right {
        let pivot = partition(array, left, right);
        if pivot != 0 {
            quick_sort_range(array, left, pivot - 1);
        }
        quick_sort_range(array, pivot + 1, right);
    }
}

fn partition<T: PartialOrd>(array: &mut [T], left: usize, right: usize) -> usize {
    // 默认枢轴为left
    let pivot = left;
    let (mut low, mut high) = (left, right);
    while low < high {
        // 找到右边第一个不大于等于 arr[pivot] 的元素
        while low < high && array[high] >= array[pivot] {
            high -= 1;
        }
        // 找到左边第一个不小于等于 arr[pivot] 的元素
        while low < high && array[low] <= array[pivot] {
            low += 1;
        }
        // 交换前面找到的两个元素
        if low != high {
            array.swap(low, high);
        }
    }
    array.swap(pivot, low);
    // 返回正确的分割位置
    low
}

#[cfg(test)]
mod tests_quick {
    use crate::sort::quick_sort;

    #[test]
    fn test_empty() {
        let mut empty: Vec<String> = vec![];
        quick_sort(&mut empty);
        assert_eq!(empty, Vec::<String>::new())
    }

    #[test]
    fn test_numbers() {
        let mut nums = vec![21, 43, 555, 12, 43, 432, 8765, 132, 6543, 11, 0, 24321];
        quick_sort(&mut nums);
        assert_eq!(nums, vec![0, 11, 12, 21, 43, 43, 132, 432, 555, 6543, 8765, 24321]);
    }

    #[test]
    fn test_strings() {
        let mut strings = vec!["Bob".to_string(), "Alice".to_string(), "Eve".to_string(), "David".to_string()];
        quick_sort(&mut strings);
        assert_eq!(strings, vec!["Alice".to_string(), "Bob".to_string(), "David".to_string(), "Eve".to_string()])
    }
}