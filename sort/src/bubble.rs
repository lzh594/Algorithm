pub fn bubble_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len <= 1 {
        return;
    }
    for i in 0..len as usize {
        let mut flag = false;
        for j in 0..(len - i - 1) as usize {
            if array[j] > array[j - 1] {
                array.swap(j - 1, j);
                flag = true;
            }
        }
        if flag {
            break;
        }
    }
}