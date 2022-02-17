/// Solution for [2047. Number of Valid Words in a Sentence](https://leetcode-cn.com/problems/number-of-valid-words-in-a-sentence/)
pub struct Solution {}

impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        let words = sentence.split_whitespace().collect::<Vec<_>>();
        let mut valid = 0;
        for &word in words.iter() {
            let word = word.chars().collect::<Vec<char>>();
            let mut hyphen = false;
            for (i, ch) in word.iter().enumerate() {
                match ch {
                    '0'..='9' => break,
                    '-' => {
                        if hyphen || i == 0 || i == word.len() - 1 {
                            break;
                        }
                        hyphen = true;
                        if word[i + 1] < 'a' || word[i + 1] > 'z' {
                            break;
                        }
                    }
                    '!' | '.' | ',' => {
                        if i != word.len() - 1 {
                            break;
                        } else {
                            valid += 1;
                        }
                    }
                    _ => {
                        if i == word.len() - 1 {
                            valid += 1;
                        }
                    }
                }
            }
        }
        valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::count_valid_words("cat and dog".to_string()), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::count_valid_words("!this  1-s b8d!".to_string()),
            0
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::count_valid_words("alice and  bob are playing stone-game10".to_string()),
            5
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            Solution::count_valid_words(
                "he bought 2 pencils, 3 erasers, and 1  pencil-sharpener.".to_string()
            ),
            6
        );
    }
}
