/// Solution for [521. Longest Uncommon Subsequence I](https://leetcode-cn.com/problems/longest-uncommon-subsequence-i/)
pub struct Solution {}

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            -1
        } else if a.len() > b.len() {
            a.len() as i32
        } else {
            b.len() as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::find_lu_slength("aba".to_string(), "cdc".to_string()),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::find_lu_slength("aaa".to_string(), "bbb".to_string()),
            3
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::find_lu_slength("aaa".to_string(), "aaa".to_string()),
            -1
        );
    }
}
