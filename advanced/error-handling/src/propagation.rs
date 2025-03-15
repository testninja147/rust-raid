use std::fs::{File, OpenOptions};
use std::io;
/// # Propagating Error with question mark `?` annotation
/// --------------------------------------------------------
///
/// Sometimes, we do not want to handle error at certain functions and just
/// propagate the error to the calling statement. Rust helps us propagating
/// errors with the help of `?` annotation.
///
/// Generally, ? annotations are used in libraries so that people who use that
/// function can decide to do whatever they want to do with the error.
///
fn read_file_or_error(path: &str) -> Result<File, io::Error> {
    // Try to open a file, or propagate the error to the parent
    let file = OpenOptions::new().read(true).open(path)?;
    println!("✅ File at path '{path}' successfully opened.");
    return Ok(file);
}

fn main() {
    let file_paths = ["README.md", "NON_EXISTENT.md"];
    for path in file_paths {
        if let Ok(_file) = read_file_or_error(path) {
            println!("File successfully returned!")
        } else {
            println!("⛔ File '{path}' could not be opened, Error Propagated!")
        }
    }
}
/*
 # OUTPUT:
 ---------

 ✅ File at path 'README.md' successfully opened.
File successfully returned!
⛔ File 'NON_EXISTENT.md' could not be opened, Error Propagated!
*/
