use std::cmp::min;
use std::collections::HashMap;

/// Solution for [1189. Maximum Number of Balloons](https://leetcode-cn.com/problems/maximum-number-of-balloons/)
pub struct Solution {}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut need = HashMap::new();
        for ch in "balloon".to_string().chars() {
            *need.entry(ch).or_insert(0) += 1;
        }
        let mut map = HashMap::new();
        for ch in text.chars() {
            if need.contains_key(&ch) {
                *map.entry(ch).or_insert(0) += 1;
            }
        }
        let mut result = i32::MAX;
        for (k, v) in map {
            result = min(result, v / need.get(&k).unwrap());
            need.remove(&k);
        }
        if result == i32::MAX || need.keys().len() != 0 {
            0
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_string()), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::max_number_of_balloons("loonbalxballpoon".to_string()),
            2
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::max_number_of_balloons("leetcode".to_string()), 0);
    }
}
