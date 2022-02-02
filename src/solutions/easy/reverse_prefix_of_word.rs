/// Solution for [2000. Reverse Prefix of Word](https://leetcode-cn.com/problems/reverse-prefix-of-word/)
pub struct Solution {}

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        match word.find(ch) {
            Some(index) => word[..=index].chars().rev().collect::<String>() + &word[index + 1..],
            _ => word,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::reverse_prefix("abcdefd".to_string(), 'd'),
            "dcbaefd".to_string()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::reverse_prefix("xyxzxe".to_string(), 'z'),
            "zxyxxe".to_string()
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::reverse_prefix("abcd".to_string(), 'z'),
            "abcd".to_string()
        );
    }
}
