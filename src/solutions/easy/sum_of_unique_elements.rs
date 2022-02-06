use std::collections::HashMap;

/// Solution for [1748. Sum of Unique Elements](https://leetcode-cn.com/problems/sum-of-unique-elements/)
pub struct Solution {}

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        nums.iter().for_each(|&x| *map.entry(x).or_insert(0) += 1);
        map.iter().filter(|(_, v)| **v == 1).map(|(&k, _)| k).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 2]), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
    }
}
