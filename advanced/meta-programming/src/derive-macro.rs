// ! to run, execute: cargo run --bin derive
use xml_derive::Serialize;

#[derive(Debug, Serialize)]
struct Student {
    id: usize,
    first_name: String,
    last_name: String,
    major: String,
}

/// # Derive Macro
/// ---------------
///
/// Derive macro is a procedural macro, that generates code from the given token
/// stream. It adds a new input interface to the #[Derive] attribute.
///
/// Some of the builtin procedural macros are as follows:
/// - Debug
/// - Display
/// - Copy
/// - Clone
///
/// We use derive macro to add complex functionality, for example serializing
/// and deserializing our data. In the example below, we have implemented a
/// xml serialization method using the custom derive macro.
///
///
/// To create a derive macro, we have to create a new library create that needs
/// to have a cargo.toml configured to add the following property:
///
/// ```toml
/// ...
///
/// [lib]
/// proc-macro = true
/// ...
///
/// ```
///
/// After creating the lib, we need to import the library to our implementation
/// part.
///
/// To know more about procedural macro, please check the following:
/// https://doc.rust-lang.org/reference/procedural-macros.html#procedural-macros
fn main() {
    println!("Derive Macro");
    let student_1 = Student {
        id: 1,
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        major: String::from("Computer Science"),
    };
    println!("Student 1: {:?}", student_1);
    println!("Serialized Data:\n{}", student_1.to_xml());
}
