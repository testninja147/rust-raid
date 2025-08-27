use clap::Parser;
use regex::Regex;
use std::{env::current_dir, path::PathBuf, process::exit};

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg()]
    url: String,

    #[arg(default_value_t = String::from("."))]
    dest: String,

    #[arg(short, default_value_t = 0)]
    thread: u8,
}

fn main() {
    let args = Args::parse();
    let url = args.url;
    let save_destination = PathBuf::from(args.dest);
    let thread = args.thread;
    let current_directory = current_dir().unwrap();
    println!("current_directory: {:?}", current_directory);
    println!("Url: {}", url);
    println!("save to: {:?}", save_destination);
    println!("Threads: {}", thread);

    // FIXME: add extensive url pattern matcher if it does not work on all cases
    let re = Regex::new(r"(?:^\s)(https?://)?(\w*\.)?").unwrap();
    if let Some(caps) = re.captures(url.as_str()) {
        println!("{:?}", caps);
        println!("Path is: {url}");
    } else {
        println!("Invalid Url parameter provided");
        exit(1);
    }
}
