/// Solution for [1576. Replace All ?'s to Avoid Consecutive Repeating Characters](https://leetcode-cn.com/problems/replace-all-s-to-avoid-consecutive-repeating-characters/)
pub struct Solution {}

impl Solution {
    fn get_discontinuous_char(a: char, b: char) -> char {
        let mut c = 'a';
        loop {
            c = match c {
                t if t == a || t == b => (c as u8 + 1) as char,
                _ => break,
            };
        }
        c
    }

    pub fn modify_string(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        s.insert(0, '#');
        s.push('#');
        for i in 1..s.len() - 1 {
            s[i] = match s[i] {
                '?' => Self::get_discontinuous_char(s[i - 1], s[i + 1]),
                _ => continue,
            };
        }
        s[1..s.len() - 1].into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::modify_string("?zs".to_string()), "azs");
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::modify_string("ubv?w".to_string()), "ubvaw");
    }
}
