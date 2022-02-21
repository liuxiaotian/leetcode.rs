/// Solution for [838. Push Dominoes](https://leetcode-cn.com/problems/push-dominoes/)
pub struct Solution {}

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut chars: Vec<char> = dominoes.chars().collect();
        let mut i = 0;
        let mut left = 'L';
        while i < chars.len() {
            let mut j = i;
            while j < chars.len() && chars[j] == '.' {
                j += 1;
            }
            let right = if j < chars.len() { chars[j] } else { 'R' };
            if left == right {
                while i < j {
                    chars[i] = right;
                    i += 1;
                }
            } else if left == 'R' && right == 'L' {
                let mut k = j - 1;
                while i < k {
                    chars[i] = 'R';
                    chars[k] = 'L';
                    i += 1;
                    k += 1;
                }
            }
            left = right;
            i = j + 1;
        }
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::push_dominoes("RR.L".to_string()),
            "RR.L".to_string()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::push_dominoes(".L.R...LR..L..".to_string()),
            "LL.RR.LLRRLL..".to_string()
        );
    }
}
