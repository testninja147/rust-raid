///
/// ! To run, enter the command :    cargo run --bin unsafe
///
/// Unsafe Rust
/// -----------
///
/// Rust is a memory safe programming language, however it's memory safety can
/// limit some low level operations such as dereferencing a raw pointe or modify
/// a mutable static variable. At this point, if we use unsafe scope, rust's
/// static analysis will be skipped.
///
/// However, using unsafe operations should still comply with ownership rules
/// and borrow checking, so using unsafe is not going to compile our code if we
/// try to borrow a moved value.
///
/// To know more about unsafe rust, you can check the official documentation
/// https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
fn main() {
    println!("Unsafe rust");

    {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        // we need unsafe block to deference a raw pointer
        // if we uncomment following lines and try to compile, it fails.

        // println!("r1 is: {}", *r1); // dereference of raw pointer is unsafe
        // println!("r2 is: {}", *r2); // raw pointers may be null, dangling or unaligned

        // accessing the dereferenced pointers is unsafe since it might already
        // have been removed or unavailable at execution time. Because of this
        // it is suggested not to use unsafe unless required and use as less
        // code as possible inside the unsafe block.
        unsafe {
            // dereferencing inside unsafe
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    // just used to remove some warnings in this scope
    #[allow(unused_unsafe, unused_variables)]
    {
        unsafe {
            // trying to borrow a moved value is still invalid using unsafe
            let x = String::from("Hello world");
            let y = x; // value is moved

            // if we uncomment and run the following line, it still fails to compile.
            // println!("{x}"); // Error: borrow of moved value: x;
        }
    }
}
