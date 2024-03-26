/// # 冒泡排序
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
/// use sort::bubble::bubble_sort;
/// let mut nums = vec![21, 43, 555, 12, 43, 432, 8765, 132, 6543, 11, 0, 24321];
/// bubble_sort(&mut nums);
/// assert_eq!(nums, vec![0, 11, 12, 21, 43, 43, 132, 432, 555, 6543, 8765, 24321]);
/// ```
pub fn bubble_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len <= 1 {
        return;
    }
    for i in 0..len as usize {
        let mut flag = false;
        for j in 0..(len - i - 1) as usize {
            if array[j] > array[j + 1] {
                array.swap(j, j+1);
                flag = true;
            }
        }
        if !flag {
            break;
        }
    }
}