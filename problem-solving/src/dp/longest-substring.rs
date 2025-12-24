//! # Longest Substring without duplicates
//!
//! Given a string `s`, find the length of the longest substring without
//! duplicate characters.
//!
//! Here, we check if the duplicate exists using the sliding window.
//!
//! we start with the hashmap that stores the character, and it's occurrence
//! index. if the duplicate is found, left index of that character is moved
//! so that the starting point of the count will be from that index + 1.
//!
//! at the end of the loop, we check if we achieved max length by comparing prev
//! max length, and updating it.

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
            if index >= left {
                left = index + 1;
            }
        }
        char_map.insert(char, right);
        max_len = max(max_len, right - left + 1);
        // TODO: uncomment to see assigned values
        // println!("character: {char}, left: {left}, right: {right}, max_len: {max_len}");
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
    fn two_items() {
        assert_eq!(length_of_longest_substring("au".to_owned()), 2);
    }
    #[test]
    fn test_for_correctness() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_owned()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_owned()), 1);
        assert_eq!(length_of_longest_substring("bbbabb".to_owned()), 2);
        assert_eq!(length_of_longest_substring("pwwkew".to_owned()), 3);
        assert_eq!(length_of_longest_substring("abcbcad".to_owned()), 4);
    }
}
