/// 全排列
///
/// # Arguments
///
/// * `nums`: 基础排列数组
///
/// returns: Vec<Vec<i32, Global>, Global>
///
/// # Examples
///
/// ```
/// use algorithm::backtracking::permute;
/// let permutation = permute(vec![1, 2, 3]);
/// assert_eq!(permutation.len(), 6);
/// assert!(permutation.contains(&vec![2, 1, 3]));
/// ```
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; nums.len()];
    backtrack(&nums, &mut current, &mut used, &mut res);
    res
}

fn backtrack(nums: &Vec<i32>, current: &mut Vec<i32>, used: &mut Vec<bool>, res: &mut Vec<Vec<i32>>) {
    if current.len() == nums.len() {
        res.push(current.clone());
        return;
    }
    for i in 0..nums.len() {
        if used[i] {
            continue;
        }
        current.push(nums[i]);
        used[i] = true;
        backtrack(nums, current, used, res);
        current.pop();
        used[i] = false;
    }
}

#[cfg(test)]
mod tests_permutation {
    use crate::backtracking::permutation::permute;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3];
        let permutation = permute(nums);
        assert_eq!(permutation.len(), 6);
        assert!(permutation.contains(&vec![1, 2, 3]));
        assert!(permutation.contains(&vec![2, 3, 1]));
        assert!(permutation.contains(&vec![3, 1, 2]));
    }
}