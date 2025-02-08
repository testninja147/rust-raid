use std::{char, collections::HashMap};

///
/// TRIE DATA STRUCTURE
/// --------------------
///
/// TRIE is a data structure that creates a tree specifically for a purpose of
/// retrieval of prefixed data. We can think of a dictionary book in which words
/// are grouped by prefix and sorted in a way that we know which word exist at
/// what position.
///
/// A Trie data structure is similar to a mapping data type or a hashmap in
/// which each hash corresponds to its data.
///
/// For example we have different words ANT, AND, DAD, and DASH
///
/// We can represent it in a TRIE data structure as follows
///
/// Example words: CAT, CAN, DAD, DAN, DAMN
///
///                   [ROOT]
///
///             C               D
///         A               A
///     T       N       D       N   M
///                                     N
/// Here first keys are `A` and `D`, where values are `N` and `A` which further
/// branches into  `D`, `T`, `S`, etc. and so on.

#[derive(Debug)]
struct TrieNode {
    value: Option<char>,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new(value: Option<char>) -> Self {
        Self {
            value,
            children: HashMap::new(),
        }
    }
    fn tree(&self) -> String {
        let value = match self.value {
            Some(v) => v.to_string(),
            None => String::new(),
        };
        if self.is_end() {
            return value;
        }
        return format!(
            "{}[{}]",
            value,
            self.children
                .values()
                .into_iter()
                .map(|node| format!("{}", node.tree()))
                .collect::<Vec<String>>()
                .join(",")
        );
    }
    fn get(&mut self, k: &char) -> Option<&TrieNode> {
        self.children.get(k)
    }
    fn get_mut(&mut self, k: &char) -> Option<&mut TrieNode> {
        self.children.get_mut(k)
    }
    fn is_end(&self) -> bool {
        self.children.is_empty()
    }
    fn insert(&mut self, word: &str) {
        // if the word is empty, simply return
        if word.is_empty() {
            return;
        }

        let key = word.chars().nth(0).unwrap();

        if let Some(child) = self.get_mut(&key) {
            // if it already contains the node with the given, just add a new key to the child
            if word.len() > 1 {
                child.insert(&word[1..]);
            }
        } else {
            // if there is no key, add a key to the node
            let mut child = TrieNode::new(Some(key));
            if word.len() > 1 {
                child.insert(&word[1..]);
            }
            self.children.insert(key, child);
        }
    }
}

fn main() {
    println!("Trie");
    let mut trie = TrieNode::new(None);
    trie.insert("CAT");
    trie.insert("CAP");
    trie.insert("CAR");
    trie.insert("CARD");
    trie.insert("DAD");
    trie.insert("DAN");
    trie.insert("DAMN");

    println!("{:20}: {}", "FULL TREE", trie.tree()); // [D[A[D,M[N],N]],C[A[T,R,P]]]

    if let Some(starts_with_d) = trie.get(&'D') {
        println!("{:20}: {}", "STARTING WITH D", starts_with_d.tree()); // D[A[D,N,M[N]]]
    }

    /*
     * Output:
     * FULL TREE           : [C[A[R[D],T,P]],D[A[M[N],D,N]]]
     * STARTING WITH D     : D[A[M[N],D,N]]
     */
}
