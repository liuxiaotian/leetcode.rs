use std::cmp;

/// Solution for [539. Minimum Time Difference](https://leetcode-cn.com/problems/minimum-time-difference/)
pub struct Solution {}

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes: Vec<i32> = time_points
            .iter()
            .map(|time_point| {
                time_point[0..2].parse::<i32>().unwrap() * 60
                    + time_point[3..].parse::<i32>().unwrap()
            })
            .collect();
        minutes.sort();
        let mut result = i32::MAX;
        for i in 1..minutes.len() {
            result = cmp::min(result, minutes[i] - minutes[i - 1]);
        }
        cmp::min(result, 24 * 60 - minutes[minutes.len() - 1] + minutes[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::find_min_difference(vec!["23:59".to_string(), "00:00".to_string()]),
            1
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::find_min_difference(vec![
                "00:00".to_string(),
                "23:59".to_string(),
                "00:00".to_string()
            ]),
            0
        )
    }
}
