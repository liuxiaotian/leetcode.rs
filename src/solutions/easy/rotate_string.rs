/// Solution for [796. Rotate String](https://leetcode-cn.com/problems/rotate-string/)
pub struct Solution {}

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        return goal.repeat(2).contains(&s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::rotate_string("abcde".to_string(), "cdeab".to_string()),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::rotate_string("abcde".to_string(), "abced".to_string()),
            false
        );
    }
}
