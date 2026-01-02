use std::slice;

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
/// <https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html>
fn main() {
    println!("Unsafe rust");
    // -------------------------------------------------------------------------
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
    // -------------------------------------------------------------------------

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
    // -------------------------------------------------------------------------
    {
        // we need to use unsafe block if we need to call unsafe function.
        // unsafe_function(); // cannot be used without unsafe block

        unsafe {
            unsafe_function(); // now this is allowed
        }

        unsafe fn unsafe_function() {
            println!("This is an unsafe function");
        }
    }
    // -------------------------------------------------------------------------
    {
        // sometimes, we need to use unsafe block when we know the code is okay,
        // but rust does not. This happens when we try to borrow non-overlapping
        // parts of the same slice.

        let mut my_vector = vec![1, 2, 3, 4, 5];
        let ptr = my_vector.as_mut_ptr();

        // if we uncomment the following line, the compilation fails
        // due to use of unsafe function without unsafe block

        // slice::from_raw_parts_mut(ptr, 2);

        unsafe {
            let slice1 = slice::from_raw_parts_mut(ptr, 2);
            println!("slice 1: {:?}", slice1); //slice 1: [1, 2]

            let slice2 = slice::from_raw_parts_mut(ptr.add(2), 4);
            println!("slice 2: {:?}", slice2); // slice 2: [3, 4, 5, 7471207]

            // the slice only has 3 valid values but we are getting slice of
            // length 4 so the fourth value will be garbage or invalid value

            // here, we can see some garbage value 7471207 since this is unsafe
            // operation and it is pointing out to some dangling values
        }
    }
}
