/// Solution for [717. 1-bit and 2-bit Characters](https://leetcode-cn.com/problems/1-bit-and-2-bit-characters/)
pub struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut index = 0;
        while index < bits.len() - 1 {
            if bits[index] == 1 {
                index += 2;
            } else {
                index += 1;
            }
        }
        index < bits.len() && bits[index] == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_one_bit_character(vec![1, 0, 0]), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::is_one_bit_character(vec![1, 1, 1, 0]), false);
    }
}
