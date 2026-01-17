//! # Depth First Search Algorithm
//!
//! Depth first search algorithm is a graph searching algorithm that starts at
//! the root node and explores as far as possible to find out whether the node
//! exists in the tree.
//!
//!
//! some example of DFS implementation are as follows:
//!
//! 1. dependency resolution
//! 2. topological sorting
//! 3. maze generation, etc.
//!
//!
//! References:
//!
//! <https://en.wikipedia.org/wiki/Depth-first_search>
//! <https://www.geeksforgeeks.org/dsa/depth-first-search-or-dfs-for-a-graph/>
//!
//!
//! The following example shows the dependency resolution using DFS.
//! In this example, a Package will behave as a Node, and its dependencies will
//! behave as its children nodes.
//!
//! example:
//!
//! app    -> [web, auth]
//! web    -> [http, logger, db]
//! auth   -> [crypto, db]
//! db     -> [os]
//! crypto -> [os]
//! logger -> [os]
//! os     -> []
//! ...
//!
//!
//! Which will look something like this
//! ```text
//!           App
//!        /       \
//!      /           \
//!    web            auth
//!   /   \       \    |    \
//! http   logger     db    crypto
//!    \        |     /      /
//!       \     |   /    /
//!            OS
//! ...
//!
//! ```

struct Package {
    id: String,
    dependencies: Vec<Package>,
}

struct DependencyGraph {
    packages: Package,
}

fn main() {
    //
    println!("Depth first search algorithm")
}
