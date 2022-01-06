/// Solution for [1185. Day of the Week](https://leetcode-cn.com/problems/day-of-the-week/)
pub struct Solution {}

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let week = [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];
        let day_of_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let mut day = day + 3;
        for i in 1..month {
            day += day_of_month[i as usize];
        }
        day += 365 * (year - 1971) + (year - 1969) / 4;
        if month >= 3 && ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0) {
            day += 1;
        }

        week[(day % 7) as usize].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::day_of_the_week(31, 8, 2019), "Saturday");
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::day_of_the_week(18, 7, 1999), "Sunday");
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::day_of_the_week(15, 8, 1993), "Sunday");
    }
}
