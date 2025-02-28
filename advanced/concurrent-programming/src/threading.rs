use std::io::Write;

use rand::seq::SliceRandom;
use tokio::time::{Duration, sleep};

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
