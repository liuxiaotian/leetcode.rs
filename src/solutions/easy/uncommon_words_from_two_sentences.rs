use std::collections::HashMap;

/// Solution for [884. Uncommon Words from Two Sentences](https://leetcode-cn.com/problems/uncommon-words-from-two-sentences/)
pub struct Solution {}

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut map = HashMap::new();
        s1.split_whitespace()
            .chain(s2.split_whitespace())
            .for_each(|word| {
                *map.entry(word).or_insert(0) += 1;
            });
        map.iter()
            .filter(|(_, &value)| value == 1)
            .map(|(&key, _)| key.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::uncommon_from_sentences(
                "this apple is sweet".to_string(),
                "this apple is sour".to_string()
            ),
            vec!["sweet".to_string(), "sour".to_string()]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::uncommon_from_sentences("apple apple".to_string(), "banana".to_string()),
            vec!["banana".to_string()]
        );
    }
}
