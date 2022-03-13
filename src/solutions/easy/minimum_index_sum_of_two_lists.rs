use std::collections::HashMap;

/// Solution for [599. Minimum Index Sum of Two Lists](https://leetcode-cn.com/problems/minimum-index-sum-of-two-lists/)
pub struct Solution {}

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        for (i, v) in list1.iter().enumerate() {
            map.insert(v, i);
        }
        let mut result = vec![];
        let mut index_sum = usize::MAX;
        for (i, v) in list2.iter().enumerate() {
            match map.get(v) {
                Some(index) => {
                    if i + index < index_sum {
                        index_sum = i + index;
                        result.clear();
                    }
                    if i + index <= index_sum {
                        result.push(v.to_string())
                    }
                }
                None => continue,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "Piatti".to_string(),
                    "The Grill at Torrey Pines".to_string(),
                    "Hungry Hunter Steakhouse".to_string(),
                    "Shogun".to_string()
                ]
            ),
            vec!["Shogun".to_string()]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "KFC".to_string(),
                    "Shogun".to_string(),
                    "Burger King".to_string()
                ]
            ),
            vec!["Shogun".to_string()]
        );
    }
}
