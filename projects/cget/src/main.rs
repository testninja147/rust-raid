use clap::Parser;
use regex::Regex;
use std::{env::current_dir, path::PathBuf, process::exit};

mod downloader;

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg()]
    url: String,

    #[arg(default_value_t = String::from("."))]
    dest: String,

    #[arg(short, default_value_t = 8)]
    threads: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = args.url;
    let threads = args.threads;
    // let mut save_destination = PathBuf::from(args.dest);
    // let current_directory = current_dir().unwrap();

    // FIXME: add extensive url pattern matcher if it does not work on all cases
    let re = Regex::new(r"https?://[^\s/$.?#].[^\s]*").unwrap();
    if let Some(_) = re.captures(url.as_str()) {
        // println!("capture: {:?}", captures);
        // println!("capture: {:?}", caps.);
        // println!("Path is: {url}");
        if let Ok(_) = downloader::download(url, threads).await {
            println!("File downloaded")
        }
    } else {
        println!("Invalid Url parameter provided");
        exit(1);
    }
}
