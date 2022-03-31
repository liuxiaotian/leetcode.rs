/// Solution for [728. Self Dividing Numbers](https://leetcode-cn.com/problems/self-dividing-numbers/)
pub struct Solution {}

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        fn is_self_dividing(num: i32) -> bool {
            let mut temp = num;
            while temp > 0 {
                let digit = temp % 10;
                if digit == 0 || num % digit != 0 {
                    return false;
                }
                temp /= 10;
            }
            true
        }
        let mut result = Vec::new();
        for num in left..=right {
            if is_self_dividing(num) {
                result.push(num);
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
            Solution::self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::self_dividing_numbers(47, 85),
            vec![48, 55, 66, 77]
        );
    }
}
