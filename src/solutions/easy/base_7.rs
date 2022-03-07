pub struct Solution {}

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut num = num;
        let mut res = String::new();
        let mut neg = false;
        if num < 0 {
            num = -1 * num;
            neg = true;
        }
        while num > 0 {
            res.push(((num % 7) as u8 + '0' as u8) as char);
            num /= 7;
        }
        if neg {
            res.push('-');
        }
        res.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::convert_to_base7(100), "202".to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::convert_to_base7(-7), "-10".to_string());
    }
}
