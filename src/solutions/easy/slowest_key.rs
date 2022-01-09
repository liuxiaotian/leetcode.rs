/// Solution for [1629. Slowest Key](https://leetcode-cn.com/problems/slowest-key/)
pub struct Solution {}

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        keys_pressed
            .chars()
            .enumerate()
            .fold(('a', 0), |(res, longest_duration), (i, ch)| match i {
                0 => (ch, release_times[0]),
                _ => {
                    let current_duration = release_times[i] - release_times[i - 1];
                    if current_duration > longest_duration
                        || (current_duration == longest_duration && ch > res)
                    {
                        (ch, current_duration)
                    } else {
                        (res, longest_duration)
                    }
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::slowest_key(vec![9, 29, 49, 50], "cbcd".to_string()),
            'c'
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::slowest_key(vec![12, 23, 36, 46, 62], "spuda".to_string()),
            'a'
        );
    }
}
