pub struct Solution {}

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        fn helper(answer_key: &Vec<char>, k: i32, ch: char) -> i32 {
            let length = answer_key.len();
            let mut result = 0;
            let mut left = 0;
            let mut right = 0;
            let mut sum = 0;
            while right < length {
                if answer_key[right] != ch {
                    sum += 1;
                }
                while left < right && sum > k {
                    if answer_key[left] != ch {
                        sum -= 1;
                    }
                    left += 1;
                }
                result = result.max(right - left + 1);
                right += 1;
            }
            result as i32
        }
        let answer_key = answer_key.chars().collect::<Vec<char>>();
        return helper(&answer_key, k, 'T').max(helper(&answer_key, k, 'F'));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_consecutive_answers("TTFF".to_string(), 2), 4);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_consecutive_answers("TFFT".to_string(), 1), 3);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::max_consecutive_answers("TTFTTFTT".to_string(), 1),
            5
        );
    }
}
