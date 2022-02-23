/// Solution for [917. Reverse Only Letters](https://leetcode-cn.com/problems/reverse-only-letters/)
pub struct Solution {}

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut window = (0, s.len() - 1);
        let mut chars: Vec<char> = s.chars().collect();
        while window.0 < window.1 {
            if chars[window.0].is_alphabetic() && chars[window.1].is_alphabetic() {
                chars.swap(window.0, window.1);
                window.0 += 1;
                window.1 -= 1;
                continue;
            }
            if !chars[window.0].is_alphabetic() {
                window.0 += 1;
            }
            if !chars[window.1].is_alphabetic() {
                window.1 -= 1;
            }
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
            Solution::reverse_only_letters("ab-cd".to_string()),
            "dc-ba".to_string()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba".to_string()
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
            "Qedo1ct-eeLg=ntse-T!".to_string()
        );
    }
}
