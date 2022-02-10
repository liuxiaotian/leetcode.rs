/// Solution for [1447. Simplified Fractions](https://leetcode-cn.com/problems/simplified-fractions/)
pub struct Solution {}

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        let mut result = Vec::new();

        fn gcd(a: i32, b: i32) -> i32 {
            if b != 0 {
                gcd(b, a % b)
            } else {
                a
            }
        }

        for denominator in 2..=n {
            for numerator in 1..denominator {
                if gcd(numerator, denominator) == 1 {
                    result.push(format!("{}/{}", numerator, denominator));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::simplified_fractions(2), vec!["1/2".to_string()])
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::simplified_fractions(3),
            vec!["1/2".to_string(), "1/3".to_string(), "2/3".to_string()]
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::simplified_fractions(4),
            vec![
                "1/2".to_string(),
                "1/3".to_string(),
                "2/3".to_string(),
                "1/4".to_string(),
                "3/4".to_string()
            ]
        )
    }
}
