use std::io::Write;

use rand::seq::SliceRandom;
use tokio::time::{Duration, sleep};

/// # Threading and Async programming in rustlang
/// ---------------------------------------------
///
/// Threading is a process of doing different tasks independently of each other.
/// Rustlang has some task executor libraries such as `tokio`, `future-rs`, etc
/// to handle async operations.
///
/// Async operations are helpful when we need to run tasks that require large
/// amount of time and the processor needs some wait time before the task is
/// executed.
///
/// Examples of async operations are as follows:
/// 1. Network Requests
/// 2. Streaming large files over http
/// 3. Processing Websocket Messages
///
/// To know more about threading, please refer to the rust async book:
/// <https://rust-lang.github.io/async-book/intro.html>
struct TextLoader {
    content: Vec<(char, bool)>,
}

impl TextLoader {
    fn new(content: String) -> Self {
        Self {
            content: content.chars().into_iter().map(|c| (c, false)).collect(),
        }
    }

    fn display_content(&self) {
        print!("\r");
        print!(
            "[{}]",
            self.content
                .clone()
                .into_iter()
                .map(|(ch, d)| { if d { ch } else { '-' } })
                .collect::<String>()
        );
        std::io::stdout().flush().unwrap();
    }
    async fn load(&mut self, wait_ms: u64) {
        let mut x = (0..self.content.len()).collect::<Vec<_>>();
        let mut rng = rand::rng();
        x.shuffle(&mut rng);

        for index in x {
            sleep(Duration::from_millis(wait_ms)).await;
            self.content[index].1 = true;
            self.display_content();
        }
    }
}

#[tokio::main]
async fn main() {
    let mut text_loader = TextLoader::new(String::from(
        "This is an animated text content which gets loaded randomly and asynchronously at different time.",
    ));

    println!("⛔using Async functions to load and wait⛔");
    text_loader.load(50).await;
}
