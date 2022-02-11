/// Solution for [1984. Minimum Difference Between Highest and Lowest of K Scores](https://leetcode-cn.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/)
pub struct Solution {}

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let k = k as usize;
        nums.sort_unstable();
        let mut result = nums[nums.len() - 1] - nums[0];
        if nums.len() > k {
            for i in 0..=nums.len() - (k as usize) {
                result = std::cmp::min(nums[i + k - 1] - nums[i], result);
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
        assert_eq!(Solution::minimum_difference(vec![90], 1), 0);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
    }
}
