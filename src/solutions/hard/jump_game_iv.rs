use std::collections::HashMap;
use std::collections::VecDeque;

/// Solution for [1345. Jump Game IV](https://leetcode-cn.com/problems/jump-game-iv/)
pub struct Solution {}

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        // BFS
        let mut map = HashMap::new();
        arr.iter().enumerate().for_each(|(i, &value)| {
            map.entry(value).or_insert(vec![]).push(i);
        });
        let mut visited = vec![false; arr.len()];
        let mut queue: VecDeque<usize> = VecDeque::new();

        let mut jumps = 0;
        let target = arr.len() - 1;
        visited[0] = true;
        queue.push_back(0);

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();
                if current == target {
                    return jumps;
                }
                let value = arr[current];
                if let Some(indices) = map.get(&value) {
                    indices.iter().for_each(|&index| {
                        if !visited[index] {
                            visited[index] = true;
                            queue.push_back(index);
                        }
                    });
                    map.remove(&value);
                }
                if current + 1 < arr.len() && !visited[current + 1] {
                    visited[current + 1] = true;
                    queue.push_back(current + 1);
                }
                if current as i32 - 1 >= 0 && !visited[current - 1] {
                    visited[current + 1] = true;
                    queue.push_back(current - 1);
                }
            }
            jumps += 1;
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::min_jumps(vec![7]), 0);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
    }
}
