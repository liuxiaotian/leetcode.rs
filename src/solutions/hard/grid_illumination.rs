use std::collections::{HashMap, HashSet};

/// Solution for [1001. Grid Illumination](https://leetcode-cn.com/problems/grid-illumination/)
pub struct Solution {}

impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut row = HashMap::new();
        let mut col = HashMap::new();
        let mut diagonal = HashMap::new();
        let mut anti_diagonal = HashMap::new();
        let mut points = HashSet::new();
        for lamp in lamps {
            if points.insert((lamp[0], lamp[1])) {
                *row.entry(lamp[0]).or_insert(0) += 1;
                *col.entry(lamp[1]).or_insert(0) += 1;
                *diagonal.entry(lamp[0] - lamp[1]).or_insert(0) += 1;
                *anti_diagonal.entry(lamp[0] + lamp[1]).or_insert(0) += 1;
            }
        }

        let mut result = Vec::new();
        fn entry(map: &HashMap<i32, i32>, line: &i32) -> bool {
            match map.get(line) {
                None => false,
                Some(&count) => count > 0,
            }
        }
        for query in queries {
            if entry(&row, &query[0])
                || entry(&col, &query[1])
                || entry(&diagonal, &(query[0] - query[1]))
                || entry(&anti_diagonal, &(query[0] + query[1]))
            {
                result.push(1);
            } else {
                result.push(0);
            }

            for x in query[0] - 1..=query[0] + 1 {
                for y in query[1] - 1..=query[1] + 1 {
                    if x < 0 || y < 0 || x >= n || y >= n {
                        continue;
                    }
                    if points.remove(&(x, y)) {
                        *row.get_mut(&x).unwrap() -= 1;
                        *col.get_mut(&y).unwrap() -= 1;
                        *diagonal.get_mut(&(x - y)).unwrap() -= 1;
                        *anti_diagonal.get_mut(&(x + y)).unwrap() -= 1;
                    }
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
            Solution::grid_illumination(
                5,
                vec![vec![0, 0], vec![4, 4]],
                vec![vec![1, 1], vec![1, 0]]
            ),
            vec![1, 0]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::grid_illumination(
                5,
                vec![vec![0, 0], vec![4, 4]],
                vec![vec![1, 1], vec![1, 1]]
            ),
            vec![1, 1]
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::grid_illumination(
                5,
                vec![vec![0, 0], vec![0, 4]],
                vec![vec![0, 4], vec![0, 1], vec![1, 4]]
            ),
            vec![1, 1, 0]
        );
    }
}
