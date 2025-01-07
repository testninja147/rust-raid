use clap::{Parser, Subcommand};
use std::io::{self, stdin, stdout, Read, Write};
mod vault;
use vault::Vault;
/// This function reads input from the standard input (stdin) and returns it as a String.
///
/// # Parameters
///
/// * `prompt` - A string that is printed to the console before reading input. This is used to
///              prompt the user for input.
///
/// # Return
///
/// * A String containing the input read from the console. The leading and trailing whitespace are
///   trimmed from the input.
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");
    input.trim().to_owned()
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    #[command(about = "Creates a new vault with the name provided or the default vault ")]
    Create {
        #[arg(short, long, default_value_t = String::from("default"))]
        name: String,
    },
    #[command(about = "Opens the vault with the name provided or the default vault ")]
    Open {
        #[arg(short, long, default_value_t = String::from("default"))]
        name: String,
    },
    Run,
    Destroy,
}

#[derive(Parser, Debug)]
#[command(version, about="Vault CLI", long_about = None)]
// #[command()]
struct Cli {
    #[command(subcommand)]
    subcommand: Option<Subcommands>,
}

fn main() {
    let command = Cli::parse();

    match command.subcommand {
        Some(Subcommands::Open { name }) => {
            println!("opening the vault: {name}");
            let password = input("Enter password:");
            let _ = Vault::open(name, password);
        }
        Some(Subcommands::Create { name }) => {
            Vault::create(name);
        }
        Some(Subcommands::Destroy) => todo!(),
        _ => vault_shell(),
    }
}

fn vault_shell() {
    loop {
        // println!("]")
        match input("[ 🔒 ]: ").as_str() {
            "create" => {
                Vault::create(input("Vault Name: "));
            }
            "open" => {
                match Vault::open(input("Vault Name:"), input("Vault Password:")) {
                    Ok(vault) => {
                        // stay inside the vault
                        println!("✅ The vault is unlocked");

                        loop {
                            match input(format!("[ 🔓🔑 <{}>]: ", vault.name.clone()).as_str())
                                .as_str()
                            {
                                "lock" | "exit" | "close" => {
                                    println!("⛔ The vault is now locked");
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(e) => {
                        println!("⛔ {e}")
                    }
                };
            }
            "exit" => {
                break;
            }
            _ => {}
        };
    }
}
