use std::{
    io::{self, Write},
    str::FromStr,
};

/**
 * It takes a parameter prompt as a &str and prints out as a prompt before
 * taking input from a user. This method is similar to input() method in
 * python.
 *
 */
pub fn raw_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut _str = String::new();
    io::stdin().read_line(&mut _str).unwrap();
    _str
}

/**
 * It takes a parameter prompt as a &str and prints out as a prompt before
 * taking input from a user. This method is similar to input() method in
 * python.
 *
 */
pub fn input(prompt: &str) -> String {
    raw_input(prompt).trim().to_owned()
}
/**
 * It takes a parameter prompt as a &str and prints out as a prompt before
 * taking input from a user. This method is similar to input() method in
 * python.
 *
 */
pub fn parse_input<T: FromStr>(prompt: &str) -> Result<T, T::Err> {
    let parsed = raw_input(prompt).trim().to_owned().parse::<T>()?;
    Ok(parsed)
}
