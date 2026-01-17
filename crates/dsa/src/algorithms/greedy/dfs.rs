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

use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Package {
    id: String,
    deps: Vec<String>, // dependencies
}

#[derive(Debug)]
struct Registry {
    installed: HashSet<String>,
    packages: HashMap<String, Package>,
}

impl Package {
    fn new(id: String, deps: Vec<String>) -> Self {
        Self { id, deps }
    }
}

impl Registry {
    fn new() -> Self {
        Self {
            installed: HashSet::new(),
            packages: HashMap::new(),
        }
    }
    fn insert(&mut self, pkg: Package) {
        self.packages.insert(pkg.id.clone(), pkg);
    }

    fn depth_first_search(
        &self,
        id: String,
        visiting: &mut HashSet<String>,
        resolved: &mut HashSet<String>,
        output: &mut Vec<String>,
    ) -> Result<(), String> {
        if resolved.contains(&id) {
            return Ok(());
        }

        // if a depends on b, and b depends on a, then we have a circular dependency
        if visiting.contains(&id) {
            return Err(format!("circular dependency detected at package {}", id));
        }

        visiting.insert(id.clone());

        let pkg = self
            .packages
            .get(&id)
            .ok_or_else(|| format!("package {} not found in registry", id))?;

        for dep in &pkg.deps {
            self.depth_first_search(dep.clone(), visiting, resolved, output)?;
        }

        visiting.remove(&id);
        resolved.insert(id.clone());
        output.push(id);

        Ok(())
    }

    fn resolve(&self, id: String) -> Result<Vec<String>, String> {
        // resolve packages
        let mut visiting = HashSet::new();
        let mut resolved = HashSet::new();
        let mut output = Vec::new();
        self.depth_first_search(id, &mut visiting, &mut resolved, &mut output)?;

        Ok(output)
    }

    fn install(&mut self, id: String) -> Result<(), String> {
        println!("{}", "-".repeat(40));
        if self.installed.contains(&id) {
            println!("Package \"{}\" is already installed", id);
            return Ok(());
        } else {
            println!("Installing package \"{}\"", id);
        }
        let deps = self.resolve(id.clone())?;
        println!("DFS Graph: {:?}", deps);

        // install sub dependencies first, and ignore self dependency from dfs
        for dep in deps.iter().filter(|&dep| dep != &id) {
            self.install(dep.to_owned())?
        }
        self.installed.insert(id.clone());
        println!("Package \"{}\" installed successfully", id);
        Ok(())
    }
}

fn main() {
    println!("Depth first search algorithm");

    let mut registry = Registry::new();

    // build a graph of packages and their dependencies (similar to a lockfile)
    registry.insert(Package::new("os".to_owned(), vec![]));
    registry.insert(Package::new("http".to_owned(), vec!["os".to_owned()]));
    registry.insert(Package::new("crypto".to_owned(), vec!["os".to_owned()]));
    registry.insert(Package::new("db".to_owned(), vec!["os".to_owned()]));
    registry.insert(Package::new("logger".to_owned(), vec!["os".to_owned()]));

    // install higher level packages which will resolve the dependencies before installing
    registry.insert(Package::new(
        "web".to_owned(),
        vec!["http".to_owned(), "logger".to_owned()],
    ));
    registry.insert(Package::new(
        "auth".to_owned(),
        vec!["db".to_owned(), "crypto".to_owned()],
    ));

    // insert app package which depends on web and auth
    registry.insert(Package::new(
        "app".to_owned(),
        vec!["web".to_owned(), "auth".to_owned()],
    ));

    // install package app
    registry.install("app".to_owned()).unwrap();
}
