use std::collections::HashMap;

/// Solution for [1. Two Sum](https://leetcode-cn.com/problems/two-sum/)
pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::new();
        for i in 0..nums.len() {
            match hashmap.get(&(target - nums[i])) {
                Some(x) => return vec![*x, i as i32],
                None => {
                    hashmap.insert(nums[i], i as i32);
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
