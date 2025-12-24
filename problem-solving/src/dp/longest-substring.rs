//! # Longest Substring without duplicates
//!
//! Given a string `s`, find the length of the longest substring without
//! duplicate characters.

use std::{cmp::max, collections::HashMap};

fn length_of_longest_substring(string: String) -> usize {
    // return the length of string if the length is 0 or 1 since it has unique elements
    if string.len() < 2 {
        return string.len();
    }

    let mut left = 0;
    let mut max_len = 0;
    let mut char_map: HashMap<char, usize> = HashMap::new();

    for (right, char) in string.chars().enumerate() {
        if let Some(&index) = char_map.get(&char) {
            if index > left {
                left = index + 1;
            }
        }
        println!("{left}, {right}");
        char_map.insert(char, right);
        max_len = max(max_len, right - left);
    }
    return max_len;
}
fn main() {
    let length = length_of_longest_substring("abcabcbb".to_owned());
    println!("length_of_the_longest_substring: {length}");
}

#[cfg(test)]
mod tests {
    use crate::length_of_longest_substring;

    #[test]
    fn empty_string() {
        assert_eq!(length_of_longest_substring("".to_owned()), 0);
    }
    #[test]
    fn single_item() {
        assert_eq!(length_of_longest_substring("a".to_owned()), 1);
    }
    #[test]
    fn test_for_correctness() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_owned()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_owned()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_owned()), 3);
        assert_eq!(length_of_longest_substring("abcbcad".to_owned()), 4);
    }
}
