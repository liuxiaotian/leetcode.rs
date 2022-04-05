/// Solution for [762. Prime Number of Set Bits in Binary Representation](https://leetcode-cn.com/problems/prime-number-of-set-bits-in-binary-representation/)
pub struct Solution {}

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut result = 0;
        for num in left..=right {
            if 0b10100010100010101100 & 1 << num.count_ones() != 0 {
                result += 1;
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
        assert_eq!(Solution::count_prime_set_bits(6, 10), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::count_prime_set_bits(10, 15), 5);
    }
}
