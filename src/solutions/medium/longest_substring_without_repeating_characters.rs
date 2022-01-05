use std::cmp::max;
use std::collections::HashMap;

/// Solution for [3. Longest Substring Without Repeating Characters](https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/)
pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hashmap = HashMap::new();
        let mut res = 0;
        let mut window = (-1, 0);
        for c in s.chars() {
            match hashmap.insert(c, window.1) {
                Some(last) => window.0 = max(window.0, last),
                None => (),
            };
            res = max(res, window.1 - window.0);
            window.1 += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
