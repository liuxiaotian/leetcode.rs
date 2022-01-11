use std::collections::HashSet;

/// Solution for [1036. Escape a Large Maze](https://leetcode-cn.com/problems/escape-a-large-maze/)
pub struct Solution {}

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        if blocked.len() < 2 {
            return true;
        }
        let blocked = blocked.iter().map(|point| (point[0], point[1])).collect();
        match Self::check(&blocked, (source[0], source[1]), (target[0], target[1])) {
            0 => true,
            1 => false,
            _ => Self::check(&blocked, (target[0], target[1]), (source[0], source[1])) != 1,
        }
    }

    fn check(blocked: &HashSet<(i32, i32)>, source: (i32, i32), target: (i32, i32)) -> u8 {
        let mut moved = HashSet::new();
        let mut queue = Vec::new();
        let mut count = blocked.len() as i32 * (blocked.len() as i32 - 1) / 2;
        queue.push(source);
        moved.insert(source);
        while !queue.is_empty() && count > 0 {
            let current = queue.pop().unwrap();
            for d in DIRECTIONS.iter() {
                let next = (current.0 + d.0, current.1 + d.1);
                if next == target {
                    return 0;
                }
                if next.0 >= 0
                    && next.0 < 1000000
                    && next.1 >= 0
                    && next.1 < 1000000
                    && !blocked.contains(&next)
                    && !moved.contains(&next)
                {
                    count -= 1;
                    queue.push(next);
                    moved.insert(next);
                }
            }
        }
        if count > 0 {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::is_escape_possible(vec![vec![0, 1], vec![1, 0]], vec![0, 0], vec![0, 2]),
            false
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::is_escape_possible(Vec::<Vec<i32>>::new(), vec![0, 0], vec![999999, 999999]),
            true
        );
    }
}
