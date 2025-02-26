/// Ownership and Borrowing in Rust
/// ------------------------------------
///
/// This code snippet covers the following:
/// - ownership
/// - borrowing
/// - lifetime and scopes
///
/// Rust's ownership is the most important feature of memory management in rust.
/// We can think of ownership as a basic ownership of our property. A property,
/// example house, can be owned by an owner at a time. If it needs to be used by
/// another person, they must buy it. This process is called borrowing in rust.
///
/// An ownership defines what owns the value and when it needs to be borrowed
/// while being used.
///
/// Rules of ownership:
/// 1. Each value needs to be owned and can be owned by only one variable.
/// 2. When value goes out of scope, it will automatically be dropped
///
/// since the value will automatically be dropped when it goes out of scope, we
/// do not need any garbage collectors in rust.
///
/// you can check https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
/// to know more about ownership and how it works in detail.
fn main() {
    // -------------------------------------------------------------------------
    // creating a scope
    // we can easily create scopes in rust just by enclosing our code snippet
    // within braces {} as the code executes , all the variables and values
    // after `}` gets dropped.
    {
        // start of the scope: ownership and lifetime
        let var = "my value"; // variable var is an owner of "my value"
        println!("the var has value: {}", var);
        println!("the var has value: {}", var);
    } // end of the scope [here, var gets dropped]

    // the var will not be found in this scope now
    // println!("the value of var is: {}", var);

    // -------------------------------------------------------------------------
    {
        // start of the scope: lifetime
        // if we assign new value to the mutable variable, it immediately drops
        // the old value and then it assigns the new value.
        let mut x = 5;
        println!("{x}");
        x = 10; // at this point  previous value `5` automatically gets dropped
        println!("{x}");

        // we can also achieve it by using immutable variable and shadowing
        let x = 5;
        println!("{x}");
        let x = 10; // here previous `x` is shadowed by this one so `5` gets dropped
        println!("{x}");
    }
    // -------------------------------------------------------------------------
    {
        // start of the scope: borrowing
        let x = String::from("Hello World");
        println!("the value of x is: {x}"); // the value of x is: Hello World

        // the String data type does not implement the copy trait so the value
        // gets moved when we try to assign to another variable dropping the
        // ownership of the old variable.
        let y = x; // now the ownership is transferred to y

        // println!("the value of x is {x}"); // Error: borrow of moved value

        println!("the value of y is: {y}"); // the value of y is: Hello World
    }
    // -------------------------------------------------------------------------
    {
        // Ownership can also be transferred by functions.
        // for example:
        fn take_ownership(s: String) {
            println!("Now, the value is owned: {s}")
            // if we want this value to be available in the previous implementation,
            // then we must pass the argument by reference rather than value.
        }

        let hello = String::from("World");
        take_ownership(hello); // here the value already moved

        // println!("hello: {hello}"); // Error: Borrow of moved value
    }
    // -------------------------------------------------------------------------
    {
        // to prevent ownership transfer, we can pass the argument by reference
        // however, when we try to borrow a value from reference, it cannot be
        // mutated or modified.
        //
        // In real life If we borrow some goods from our friends, we have to
        // return as it is.
        fn do_not_take_ownership(s: &String) {
            // s.push_str("!!"); // Error: cannot borrow *s as mutable as it is
            // behind the shared reference

            println!("Now, the value is owned: {s}")
            // if we want this value to be available in the previous implementation,
            // then we must pass the argument by reference rather than value.
        }

        let mut hello = String::from("World");
        hello.push_str("!");
        do_not_take_ownership(&hello); // here the value already moved

        println!("hello: {hello}"); // the value is not borrowed
    }
    // -------------------------------------------------------------------------
    {
        // we can also retain ownership by returning values from function
        fn retain_ownership(s: String) -> String {
            println!("Now, the value is owned: {s}");
            // if we want this value to be available in the previous implementation,
            // then we must pass the argument by reference rather than value.

            // returns s
            s
        }

        let hello = String::from("World");
        let hello = retain_ownership(hello); // here the value is retained

        println!("retained hello: {hello}");
    }
    // -------------------------------------------------------------------------
}
