/// Solution for [1332. Remove Palindromic Subsequences](https://leetcode-cn.com/problems/remove-palindromic-subsequences/)
pub struct Solution {}

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s == s.chars().rev().collect::<String>() {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::remove_palindrome_sub("ababa".to_string()), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::remove_palindrome_sub("abb".to_string()), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::remove_palindrome_sub("baabb".to_string()), 2);
    }
}
