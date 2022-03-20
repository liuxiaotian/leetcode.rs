use std::collections::VecDeque;

/// Solution for [2039. The Time When the Network Becomes Idle](https://leetcode-cn.com/problems/the-time-when-the-network-becomes-idle/)
pub struct Solution {}

impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let mut graph = vec![vec![]; patience.len()];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }
        let mut queue = VecDeque::new();
        let mut visited = vec![false; patience.len()];
        queue.push_back(0 as usize);
        visited[0] = true;
        let mut distance = 1;
        let mut result = 0;
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let front = queue.pop_front().unwrap();
                for &neighbor in &graph[front] {
                    if visited[neighbor] {
                        continue;
                    }
                    queue.push_back(neighbor);
                    let time = patience[neighbor] * ((2 * distance - 1) / patience[neighbor])
                        + 2 * distance
                        + 1;
                    if time > result {
                        result = time;
                    }
                    visited[neighbor] = true;
                }
            }
            distance += 1;
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
            Solution::network_becomes_idle(vec![vec![0, 1], vec![1, 2]], vec![0, 2, 1]),
            8
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::network_becomes_idle(
                vec![vec![0, 1], vec![0, 2], vec![1, 2]],
                vec![0, 10, 10]
            ),
            3
        );
    }
}
