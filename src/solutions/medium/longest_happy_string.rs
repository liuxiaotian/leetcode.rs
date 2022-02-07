/// Solution for [1405. Longest Happy String](https://leetcode-cn.com/problems/longest-happy-string/)
pub struct Solution {}

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut abc = [(a, 'a'), (b, 'b'), (c, 'c')];
        let mut result = vec![];
        loop {
            abc.sort_unstable_by(|left, right| right.0.cmp(&left.0));
            let mut not_push = true;
            for (cnt, ch) in abc.iter_mut() {
                if *cnt == 0
                    || result.len() >= 2
                        && *ch == result[result.len() - 2]
                        && *ch == result[result.len() - 1]
                {
                    continue;
                }
                result.push(*ch);
                not_push = false;
                *cnt -= 1;
                break;
            }
            if not_push {
                break;
            }
        }
        result.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_diverse_string(1, 1, 7),
            "ccaccbcc".to_string()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_diverse_string(7, 1, 0),
            "aabaa".to_string()
        );
    }
}
