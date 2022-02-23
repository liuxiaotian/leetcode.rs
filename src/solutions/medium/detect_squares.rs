use std::collections::HashMap;

/// Solution for [2013. Detect Squares](https://leetcode-cn.com/problems/detect-squares/)
pub struct DetectSquares {
    x2y_cnt_map: HashMap<i32, HashMap<i32, i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {
    pub fn new() -> Self {
        DetectSquares {
            x2y_cnt_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, point: Vec<i32>) {
        *self
            .x2y_cnt_map
            .entry(point[0])
            .or_insert(HashMap::new())
            .entry(point[1])
            .or_insert(0) += 1;
    }

    pub fn count(&self, point: Vec<i32>) -> i32 {
        let mut result = 0;
        if let Some(y_cnt_map) = self.x2y_cnt_map.get(&point[0]) {
            for (&y, &cnt) in y_cnt_map.iter() {
                if y == point[1] {
                    continue;
                }
                let size = y - point[1];
                for x in [point[0] + size, point[0] - size] {
                    if let Some(other_y_cnt_map) = self.x2y_cnt_map.get(&x) {
                        let (cnt1, cnt2) =
                            (other_y_cnt_map.get(&y), other_y_cnt_map.get(&point[1]));
                        if cnt1.is_none() || cnt2.is_none() {
                            continue;
                        }
                        let (&a, &b) = (cnt1.unwrap(), cnt2.unwrap());
                        result += cnt * a * b;
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
        let mut detect_squares = DetectSquares::new();
        detect_squares.add(vec![3, 10]);
        detect_squares.add(vec![11, 2]);
        detect_squares.add(vec![3, 2]);
        assert_eq!(detect_squares.count(vec![11, 10]), 1);
        assert_eq!(detect_squares.count(vec![14, 8]), 0);
        detect_squares.add(vec![11, 2]);
        assert_eq!(detect_squares.count(vec![11, 10]), 2);
    }
}
