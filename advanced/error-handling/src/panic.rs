/// # Panic - an unrecoverable error
/// ---------------------------------
///
/// Unrecoverable errors are those errors that abort the execution of the
/// program. In rust, we use `panic!()` macro to throw unrecoverable error. In
/// some cases the code automatically panics for example trying to access a
/// non-existent index of an array.
///
/// Below is an example of an unrecoverable error.

fn main() {
    println!("Panic Examples");

    // A basic panic.
    // TODO: please uncomment line below to see panic message
    // panic!("⛔ The program panics here. ⛔")

    // we can also get panic when we try to access non-existent index of an
    // array

    let _primes = [2, 3, 5, 7, 11];
    // TODO: please uncomment line below to see panic message
    for x in 0..6 {
        // the code panics at 5th iteration
        println!("Prime at index 5 is: {}", _primes[x]);
    }
    // thread 'main' panicked at advanced/error-handling/src/panic.rs:25:45:
    // index out of bounds: the len is 5 but the index is 5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
