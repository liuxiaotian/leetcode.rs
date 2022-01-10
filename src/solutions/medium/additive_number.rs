pub struct Solution {}

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        for second_start in 1..num.len() - 1 {
            if second_start > 1 && num.chars().nth(0).unwrap() == '0' {
                break;
            }
            for second_end in second_start + 1..num.len() {
                if second_end - second_start > 1 && num.chars().nth(second_start).unwrap() == '0' {
                    break;
                }
                let mut third_start = second_end;
                let mut first_num = num[..second_start].parse::<u64>().unwrap();
                let mut second_num = num[second_start..second_end].parse::<u64>().unwrap();
                loop {
                    let third_num = first_num + second_num;
                    let third_end = third_start + third_num.to_string().len();
                    if third_end > num.len() {
                        break;
                    }
                    if !num[third_start..third_end].eq(third_num.to_string().as_str()) {
                        break;
                    }
                    if third_end == num.len() {
                        return true;
                    }
                    first_num = second_num;
                    second_num = third_num;
                    third_start = third_end;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_additive_number("112358".to_string()), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::is_additive_number("199100199".to_string()), true);
    }
}
