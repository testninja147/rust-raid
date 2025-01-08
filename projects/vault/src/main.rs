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

fn message_box(message: impl ToString) {
    println!("+{}+", "-".repeat(78));
    println!("|{:^78}|", message.to_string());
    println!("+{}+", "-".repeat(78));
}

fn main() {
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
                            message_box("list | get <key> | set <key> <val> | delete <key> | lock");
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
                                &["list"] => {
                                    println!("🚀 list");
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
