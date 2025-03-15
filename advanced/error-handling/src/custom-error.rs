/// # Custom Errors
/// ----------------
///
/// Custom Errors are custom made errors when standard library errors are not
/// enough to represent errors. We generally create custom error for new data
/// type or operation that is different than  that from the std crate.
///
/// Some places where custom error can be defined are as follows:
/// - Libraries
/// - Web Servers
/// - Parsers, etc.

#[allow(dead_code)]
#[derive(Debug)]
enum HttpErrors {
    NotFound,
    PermissionDenied,
    Unacceptable,
    ServerError,
}

/// This is a mock fetch request that just returns custom NotFound Error
fn fetch(_path: &str) -> Result<String, HttpErrors> {
    Err(HttpErrors::NotFound)
}
fn main() {
    // Handling Http Errors
    match fetch("/api/users/1/") {
        Ok(response) => {
            println!("response: {response}");
        }
        // Catches Custom Error: NotFound
        Err(HttpErrors::NotFound) => {
            println!("⛔ Custom Error: Resource Not Found in the database")
        }
        // Catches all other errors
        Err(_) => {
            println!("⛔ Custom Error: Other Errors")
        }
    };
}
