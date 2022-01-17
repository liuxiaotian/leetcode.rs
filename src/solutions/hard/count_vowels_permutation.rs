/// Solution for [1220. Count Vowels Permutation](https://leetcode-cn.com/problems/count-vowels-permutation/)
pub struct Solution {}

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let m = 1_000_000_007;
        let mut dp = vec![1; 5];
        for _i in 1..n {
            let tp = vec![
                dp[1] + dp[2] + dp[4],
                dp[0] + dp[2],
                dp[1] + dp[3],
                dp[2],
                dp[2] + dp[3],
            ];
            dp = tp.iter().map(|x| x % m).collect();
        }
        (dp.iter().sum::<i64>() % m) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::count_vowel_permutation(1), 5);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::count_vowel_permutation(2), 10);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::count_vowel_permutation(5), 68);
    }
}
