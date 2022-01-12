/// Solution for [334. Increasing Triplet Subsequence](https://leetcode-cn.com/problems/increasing-triplet-subsequence/)
pub struct Solution {}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut tri_0 = i32::MAX;
        let mut tri_1 = i32::MAX;
        for &num in nums.iter() {
            if num <= tri_0 {
                tri_0 = num;
            } else if num <= tri_1 {
                tri_1 = num;
            } else {
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
        assert_eq!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
    }
}
