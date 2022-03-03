/// Solution for [258. Add Digits](https://leetcode-cn.com/problems/add-digits/)
pub struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;
        let mut result = 0;
        while num > 0 {
            result += num % 10;
            num /= 10;
            if num == 0 && result >= 10 {
                num = result;
                result = 0;
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
        assert_eq!(Solution::add_digits(38), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::add_digits(0), 0);
    }
}
