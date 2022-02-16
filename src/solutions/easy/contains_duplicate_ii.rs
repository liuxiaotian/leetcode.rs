use std::collections::HashSet;

/// Solution for [219. Contains Duplicate II](https://leetcode-cn.com/problems/contains-duplicate-ii/)
pub struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut set = HashSet::new();
        let k = k as usize;
        for i in 0..nums.len() {
            if i > k {
                set.remove(&nums[i - k - 1]);
            }
            if !set.insert(nums[i]) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
            true
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        );
    }
}
