/// Solution for [2029. Stone Game IX](https://leetcode-cn.com/problems/stone-game-ix/)
pub struct Solution {}

impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut cnt = vec![0; 3];
        for value in stones {
            cnt[value as usize % 3] += 1;
        }
        if cnt[0] % 2 == 0 {
            cnt[1] >= 1 && cnt[2] >= 1
        } else {
            cnt[1] - cnt[2] > 2 || cnt[2] - cnt[1] > 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::stone_game_ix(vec![2, 1]), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::stone_game_ix(vec![2]), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::stone_game_ix(vec![5, 1, 2, 4, 3]), false);
    }
}
