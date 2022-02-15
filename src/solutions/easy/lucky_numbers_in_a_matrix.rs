use std::collections::HashSet;

/// Solution for [1380. Lucky Numbers in a Matrix](https://leetcode-cn.com/problems/lucky-numbers-in-a-matrix/)
pub struct Solution {}

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut set = HashSet::new();
        for col in 0..matrix.len() {
            let mut min_row: usize = 0;
            for row in 1..matrix[col].len() {
                if matrix[col][row] < matrix[col][min_row] {
                    min_row = row;
                }
            }
            set.insert((col, min_row));
        }
        for row in 0..matrix[0].len() {
            let mut max_col: usize = 0;
            for col in 1..matrix.len() {
                if matrix[col][row] > matrix[max_col][row] {
                    max_col = col;
                }
            }
            if !set.insert((max_col, row)) {
                result.push(matrix[max_col][row]);
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
        assert_eq!(
            Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
            vec![15]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::lucky_numbers(vec![
                vec![1, 19, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ]),
            vec![12]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]]),
            vec![7]
        );
    }
}
