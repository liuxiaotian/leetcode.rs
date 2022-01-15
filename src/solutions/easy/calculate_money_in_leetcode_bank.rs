/// Solution for [1716. Calculate Money in Leetcode Bank](https://leetcode-cn.com/problems/calculate-money-in-leetcode-bank/)
pub struct Solution {}

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut week = 1;
        let mut day = 1;
        let mut money = 0;
        let mut n = n;
        while n > 0 {
            n -= 1;
            money += week + day - 1;
            day += 1;
            if day == 8 {
                day = 1;
                week += 1;
            }
        }
        money
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::total_money(4), 10);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::total_money(10), 37);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::total_money(20), 96);
    }
}
