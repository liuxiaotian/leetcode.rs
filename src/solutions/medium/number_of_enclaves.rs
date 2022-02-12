use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = HashSet::new();
        fn dfs(grid: &Vec<Vec<i32>>, visited: &mut HashSet<(i32, i32)>, row: i32, col: i32) {
            if row < 0
                || row >= grid.len() as i32
                || col < 0
                || col >= grid[0].len() as i32
                || grid[row as usize][col as usize] == 0
                || visited.contains(&(row, col))
            {
                return;
            }
            visited.insert((row, col));
            dfs(grid, visited, row + 1, col);
            dfs(grid, visited, row - 1, col);
            dfs(grid, visited, row, col + 1);
            dfs(grid, visited, row, col - 1);
        }
        for i in 0..grid.len() {
            dfs(&grid, &mut visited, i as i32, 0);
            dfs(&grid, &mut visited, i as i32, grid[0].len() as i32 - 1);
        }
        for i in 1..grid[0].len() - 1 {
            dfs(&grid, &mut visited, 0, i as i32);
            dfs(&grid, &mut visited, grid.len() as i32 - 1, i as i32);
        }
        let mut result = 0;
        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() - 1 {
                if grid[i][j] == 1 && !visited.contains(&(i as i32, j as i32)) {
                    result += 1;
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
        assert_eq!(
            Solution::num_enclaves(vec![
                vec![0, 0, 0, 0],
                vec![1, 0, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0]
            ]),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::num_enclaves(vec![
                vec![0, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 0]
            ]),
            0
        );
    }
}
