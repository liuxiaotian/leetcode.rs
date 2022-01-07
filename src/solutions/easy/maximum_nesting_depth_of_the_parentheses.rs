/// Solution for [1614. Maximum Nesting Depth of the Parentheses](https://leetcode-cn.com/problems/maximum-nesting-depth-of-the-parentheses/)
pub struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        s.chars()
            .fold((0, 0), |(res, depth), c| match c {
                '(' => (res.max(depth + 1), depth + 1),
                ')' => (res, depth - 1),
                _ => (res, depth),
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_depth("(1)+((2))+(((3)))".to_string()), 3);
    }
}
