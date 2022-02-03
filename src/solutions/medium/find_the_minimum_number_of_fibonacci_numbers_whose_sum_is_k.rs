/// Solution for [1414. Find the Minimum Number of Fibonacci Numbers Whose Sum Is K](https://leetcode-cn.com/problems/find-the-minimum-number-of-fibonacci-numbers-whose-sum-is-k/)
pub struct Solution {}

impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut fibs = Vec::new();
        let mut num = (1, 1);
        while num.1 <= k {
            fibs.push(num.1);
            num = (num.1, num.0 + num.1);
        }
        fibs.reverse();
        let mut result = 0;
        let mut k = k;
        for fib in fibs {
            if k >= fib {
                k -= fib;
                result += 1;
                if k <= 0 {
                    break;
                }
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
        assert_eq!(Solution::find_min_fibonacci_numbers(7), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::find_min_fibonacci_numbers(10), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::find_min_fibonacci_numbers(19), 3);
    }
}
