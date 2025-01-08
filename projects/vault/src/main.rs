use clap::{Parser, Subcommand};
use std::io::{stdin, stdout, Write};
mod vault;
use rpassword::prompt_password;
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
            let password = input("Enter password [hidden]: ");
            let _ = Vault::open(name, password);
        }
        Some(Subcommands::Create { name }) => {
            Vault::create(name);
        }
        Some(Subcommands::Destroy) => todo!(),
        _ => vault_shell(),
    }
}
fn message_box(message: &str) {
    println!("+{}+", "-".repeat(78));
    println!("|{:^78}|", message);
    println!("+{}+", "-".repeat(78));
}

fn vault_shell() {
    loop {
        message_box("open | create | exit");

        match input("🔒: ").as_str() {
            "create" => {
                Vault::create(input("Vault Name: "));
            }
            "open" => {
                let name = input("Enter Vault Name: ");
                let password = prompt_password("Enter Password [hidden]: ").unwrap();
                match Vault::open(name, password) {
                    Ok(mut vault) => {
                        println!("✅ The vault is unlocked");
                        loop {
                            message_box("list | get | set | delete | lock | exit | close");
                            match input(format!("[ {} ] 🔓: ", vault.name.clone()).as_str())
                                .split(" ")
                                .collect::<Vec<&str>>()
                                .as_slice()
                            {
                                &["lock" | "exit" | "close", ..] => {
                                    println!("⛔ The vault is now locked");
                                    break;
                                }
                                &["set", k, v] => {
                                    vault.set(k.to_owned(), v.to_owned());
                                }
                                &["get", k] => {
                                    vault.get(k.to_owned());
                                }
                                &["delete", k] => {
                                    println!("🚀 delete");
                                    todo!(" delete key is not yet implemented")
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
