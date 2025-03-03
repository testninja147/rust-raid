// ! to run, execute: cargo run --bin result
use std::fs::File;
use std::io::stdin;

/// # Recoverable Errors and Result Enum
/// -------------------------------------
///
/// Recoverable errors are those errors that can be handled and the program can
/// still function with those errors. This kind of errors are generally known
/// errors and can easily be handled by changing the behavior of execution on
/// different conditions.
///
/// When we use an enum `Result<T,E>`, it will either return the `OK` response
/// with the desired type `T` or an `Error` instance with the type `E`.
///
/// For Example, We wanted to open a file named users.txt, however if the file
/// does not exist, we want to display and error message.
/// In this case `std::fs::File` crate
/// will give us  either file instance or an error instance so that we can
/// handle it accordingly.

fn main() {
    // Example 1: File Handling with `Result` enum
    {
        let file = File::open("random_file.txt");
        match file {
            Ok(_file) => {
                println!("The file is successfully opened");
            }
            Err(e) => {
                println!("The file could not be opened, Error:\n{}", e);
            }
        }
        // The file could not be opened, Error:
        // No such file or directory (os error 2)
    }

    // Example 2: We can also use `unwrap` and `unwrap_or_else` to handle some
    //  kind of error. For example: We try to get an input from the console and
    // convert it to integer. If the input string is not an integer, we set the
    // default value to 0.
    {
        let mut input_string = String::new();
        println!("Enter a number: ");

        // unwrap example (Error is not handled here)
        stdin().read_line(&mut input_string).unwrap();
        println!("Input string is : {}", input_string);

        // `unwrap_or` example: handles error, but we never know that the
        // error occured in the first place since it automatically returns the
        // default value on error.
        let parsed_int = input_string.trim().parse::<i128>().unwrap_or(0);
        println!("The parsed integer is : {}", parsed_int);

        // `unwrap_or_else` example
        // Here, the error is handled, also, we can perform specific actions
        // depending upon which error occured.
        let parsed_int = input_string
            .trim()
            .parse::<i128>()
            .unwrap_or_else(|err| -> i128 {
                return match err.kind() {
                    std::num::IntErrorKind::InvalidDigit => {
                        println!("Could not parse the integer from the given input, fallback to 0");
                        0
                    }
                    _ => {
                        println!("Did not know the exact error, fallback to -1");
                        -1
                    }
                };
            });
        println!("The parsed integer is : {}", parsed_int);

        // The file could not be opened, Error:
        // No such file or directory (os error 2)
    }
}
