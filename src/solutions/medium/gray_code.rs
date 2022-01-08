/// Solution for [89. Gray Code](https://leetcode-cn.com/problems/gray-code/)
pub struct Solution {}

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        (0..1 << n).map(|i| (i >> 1) ^ i).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::gray_code(1), vec![0, 1]);
    }
}
