//! # Find nth distinct number in the array
//!
//! To run/test, please run the following commands in your terminal
//!
//! ```sh
//! cargo run --bin binary_tree
//! ```
//!
//! ```sh
//! cargo test --bin binary_tree
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
use std::collections::HashMap;

use common::parse_input;

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
        let mut counter = pos;
        let mut hash_map = HashMap::new();
        array.iter().for_each(|v| {
            *hash_map.entry(v).or_insert(0) += 1;
        });
        for &k in &array {
            let &v = hash_map.get(&k).unwrap();
            if v == 1 {
                counter -= 1;
                if counter == 0 {
                    println!("\nThe {pos}{} distinct item is {k} ", suffix(pos));
                    return;
                }
            }
        }
        println!(
            "There is no {pos}{} distinct item in the array.",
            suffix(pos)
        );
    }
}
