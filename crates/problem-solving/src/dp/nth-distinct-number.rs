//! # Find nth distinct number in the array
//!
//! To run/test, please run the following commands in your terminal
//!
//! ```sh
//! cargo run --bin nth-distinct-number
//! ```
//!
//! ```sh
//! cargo test --bin nth-distinct-number
//! ```
//!
//! To solve this problem, we must first create a hashmap to calculate all the
//! numbers that stores, number, and its repetition.
//!
//! The above operation takes time complexity of O(n).
//!
//! in the second loop, we will get the value of key by iterating over the
//! array, and if the value is 1, that means it is a distinct number. At this
//! point we decrease the value of counter by 1. once it reaches 0, we select
//! the key as the nth number.
//!
//!
use std::{collections::HashMap, hash::Hash};

use common::parse_input;

fn get_nth_distinct_number<T: Eq + Hash + Copy>(array: Vec<T>, pos: usize) -> Result<T, ()> {
    let mut counter = pos;
    let mut hash_map = HashMap::new();
    for &k in &array {
        *hash_map.entry(k).or_insert(0) += 1;
    }
    for k in array.into_iter() {
        let count = hash_map[&k];
        if count == 1 {
            counter -= 1;
            if counter == 0 {
                return Ok(k);
            }
        }
    }
    Err(())
}

fn main() {
    println!("Nth distinct number");
    let array = vec![1, 5, 2, 7, 4, 0, 3, 4, 1, 3, 5, 6, 3, 1, 8];
    println!("Array: {array:?}");

    let suffix = |pos| match pos {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    if let Ok(pos) = parse_input::<usize>("Enter value of n: ") {
        if let Ok(num) = get_nth_distinct_number(array, pos) {
            println!("\nThe {pos}{} distinct item is {num} ", suffix(pos));
        } else {
            println!(
                "There is no {pos}{} distinct item in the array.",
                suffix(pos)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_nth_distinct_number;

    #[test]
    fn check_existing() {
        let array = vec![1, 5, 2, 7, 4, 0, 3, 4, 1, 3, 5, 6, 3, 1, 8];
        assert_eq!(get_nth_distinct_number(array.clone(), 2), Ok(7));
        assert_eq!(get_nth_distinct_number(array, 5), Ok(8));
    }

    #[test]
    fn check_error() {
        let array = vec![1, 5, 2, 7, 4, 0, 3, 4, 1, 3, 5, 6, 3, 1, 8];
        assert_eq!(get_nth_distinct_number(array, 6), Err(()));
    }
}
