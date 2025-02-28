use std::io::Write;

use futures::future::join_all;
use rand::seq::SliceRandom;
use tokio::{
    spawn,
    time::{Duration, sleep},
};

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
        // let step = (wait * 1000000) / (self.content.len() as u64);

        let mut x = (0..self.content.len()).collect::<Vec<_>>();
        let mut rng = rand::rng();
        x.shuffle(&mut rng);

        for index in x {
            sleep(Duration::from_millis(wait_ms)).await;
            self.content[index].1 = true;
            self.display_content();
        }
    }

    fn load_and_spawn(&mut self, max_wait: u64) {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    // let handles = (0..10)
    //     .into_iter()
    //     .map(|x| spawn(test(x)))
    //     .collect::<Vec<_>>();

    // let _ = join_all(handles).await;

    let mut text_loader = TextLoader::new(String::from(
        "This is an animated text content which gets loaded randomly and asynchronously at different time.",
    ));

    println!("⛔using Async functions to load and wait⛔");
    text_loader.load(50).await;
}
