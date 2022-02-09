/// Solution for [2006. Count Number of Pairs With Absolute Difference K](https://leetcode-cn.com/problems/count-number-of-pairs-with-absolute-difference-k/)
pub struct Solution {}

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if (nums[i] - nums[j]).abs() == k {
                    result += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::count_k_difference(vec![1, 2, 2, 1], 1), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::count_k_difference(vec![1, 3], 3), 0);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
    }
}
