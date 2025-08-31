use clap::Parser;
use regex::Regex;
use std::{env::current_dir, path::PathBuf, process::exit};

mod downloader;
use downloader::Downloader;

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg()]
    url: String,

    #[arg(default_value_t = String::from("."))]
    dest: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = args.url;

    // FIXME: add extensive url pattern matcher if it does not work on all cases
    let re = Regex::new(r"https?://[^\s/$.?#].[^\s]*").unwrap();
    if let Some(_) = re.captures(url.as_str()) {
        let mut downloader = Downloader::new(&url);
        if let Ok(_) = downloader.download().await {
            println!("Download Complete!")
        }
    } else {
        println!("Invalid Url parameter provided");
        exit(1);
    }
}
