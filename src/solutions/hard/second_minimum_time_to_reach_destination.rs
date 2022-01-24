pub use std::collections::VecDeque;

/// Solution for [2045. Second Minimum Time to Reach Destination](https://leetcode-cn.com/problems/second-minimum-time-to-reach-destination/)
pub struct Solution {}

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let mut graph = vec![vec![]; n as usize + 1];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }
        let mut path = vec![(i32::MAX, i32::MAX); n as usize + 1];
        let mut queue = VecDeque::new();
        queue.push_back((1, 0));
        while path[n as usize].1 == i32::MAX {
            let (current, d) = queue.pop_front().unwrap();
            for &next in graph[current].iter() {
                let d = d + 1;
                if d < path[next].0 {
                    path[next].0 = d;
                    queue.push_back((next, d));
                } else if d > path[next].0 && d < path[next].1 {
                    path[next].1 = d;
                    queue.push_back((next, d));
                }
            }
        }
        let mut result = 0;
        for _ in 0..path[n as usize].1 {
            if result % (change * 2) >= change {
                result += change * 2 - result % (change * 2);
            }
            result += time;
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
            Solution::second_minimum(
                5,
                vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]],
                3,
                5
            ),
            13
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::second_minimum(2, vec![vec![1, 2]], 3, 2), 11);
    }
}
