/// Solution for [747. Largest Number At Least Twice of Others](https://leetcode-cn.com/problems/largest-number-at-least-twice-of-others/)
pub struct Solution {}

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut index = 0;
        let mut largest = (-1, -1);
        for (i, &num) in nums.iter().enumerate() {
            if num > largest.0 {
                largest.1 = largest.0;
                largest.0 = num;
                index = i as i32;
            } else if num > largest.1 {
                largest.1 = num;
            }
        }
        if largest.0 >= largest.1 * 2 {
            index
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::dominant_index(vec![3, 6, 1, 0]), 1);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::dominant_index(vec![1, 2, 3, 4]), -1);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::dominant_index(vec![1]), 0);
    }
}
