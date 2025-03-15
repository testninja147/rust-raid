# Advanced concepts in Rust


This section helps you learn advanced concepts to make you more comfortable in
rust. The following topics will be discussed in this section:


1. Memory Management
   - [Ownership, borrowing, and Lifetimes](memory-management/src/ownership.rs) `cargo run --bin ownership`
   - [Unsafe Rust](memory-management/src/unsafe.rs)
2. Type System and Generics
   - [Generic Types](types-and-generics/src/generics.rs) `cargo run --bin generics`
   - [Trait Objects and Dynamic Dispatch](types-and-generics/src/traits.rs) `cargo run --bin traits`
   - Associated types and Generic Type parameters
   - Lifetime Sub-typing
3. Concurrency and Parallelism
   - `Async/Await` and `Futures`
   - Task Executors
   - Concurrency
4. Macros and Meta programming
   - [`macro_rules!`](meta-programming/src/macro-rules.rs) `cargo run --bin macro`
   - [Derive Macros](meta-programming/src/derive-macro.rs) `cargo run --bin derive`
   - [Building Domain-Specific Languages (DSL)](meta-programming/src/dsl.rs) `cargo run --bin dsl`
5. Low level and systems programming
   - [Conditional Compilation](systems-programming/src/conditional-compilation.rs) `cargo run --bin cc`
   - [Inline Assembly](systems-programming/src/inline-assembly.rs) `cargo run --bin assembly`
   - Foreign Function Interface (FFI)
   - Embedded rust and Bare-metal programming
6. Error handling and patterns
   - [Unrecoverable error and `panic!` macro](error-handling/src/panic.rs) `cargo run --bin panic`
   - [Recoverable error and `Result` enum](error-handling/src/result.rs) `cargo run --bin result`
   - Advanced Error Handling
     - [Propagating Errors with `?` operator](error-handling/src/propagation.rs) `cargo run --bin propagation`
     - [Custom Errors](error-handling/src/custom-error.rs) `cargo run --bin custom-error`
   - Dependency Injection patterns in rust
7. Specialized topics
   - Writing a custom allocator
   - Self-referential structs (`box`, `rc`, `Arc`)
