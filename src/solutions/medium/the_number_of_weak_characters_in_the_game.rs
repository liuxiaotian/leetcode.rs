/// Solution for [1996. The Number of Weak Characters in the Game](https://leetcode-cn.com/problems/the-number-of-weak-characters-in-the-game/)
pub struct Solution {}

impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_unstable_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));
        let mut max_defense = 0;
        let mut result = 0;
        for property in properties.iter() {
            if property[1] < max_defense {
                result += 1;
            } else {
                max_defense = property[1];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
            0
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]),
            1
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]]),
            1
        );
    }
}
