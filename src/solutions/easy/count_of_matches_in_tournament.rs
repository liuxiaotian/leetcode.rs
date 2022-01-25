pub struct Solution {}

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut n = n;
        let mut matches = 0;
        while n != 1 {
            matches += n / 2;
            n = match n % 2 {
                1 => (n - 1) / 2 + 1,
                _ => n / 2,
            };
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::number_of_matches(7), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::number_of_matches(14), 13);
    }
}
