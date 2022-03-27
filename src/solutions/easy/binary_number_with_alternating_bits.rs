pub struct Solution {}

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let a = n ^ (n >> 1);
        a & (a + 1) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(Solution::has_alternating_bits(5), true);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::has_alternating_bits(7), false);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::has_alternating_bits(11), false);
    }
}
