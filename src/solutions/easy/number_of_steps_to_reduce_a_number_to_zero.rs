pub struct Solution {}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num = num;
        let mut step = 0;
        while num != 0 {
            num = match num % 2 {
                0 => num / 2,
                _ => num - 1,
            };
            step += 1;
        }
        step
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::number_of_steps(14), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::number_of_steps(8), 4);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}
