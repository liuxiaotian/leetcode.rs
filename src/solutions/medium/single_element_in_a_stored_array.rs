/// Solution for [540. Single Element in a Sorted Array](https://leetcode-cn.com/problems/single-element-in-a-sorted-array/)
pub struct Solution {}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut low: usize = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let mid = (high - low) / 2 + low;
            if nums[mid] == nums[mid ^ 1] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        nums[low]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
            2
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
            10
        )
    }
}
