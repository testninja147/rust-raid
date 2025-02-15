# Advanced concepts in Rust


This section helps you learn advanced concepts to make you more comfortable in
rust. The following topics will be discussed in this section:


1. Memory Management
   - [Ownership, borrowing, and Lifetimes](memory-management/ownership.rs) `cargo run --bin ownership`
   - [Unsafe Rust](memory-management/unsafe.rs)
2. Type System and Generics
   - Trait Objects and Dynamic Dispatch
   - Associated types and Generic Type parameters
   - Lifetime Sub-typing
3. Concurrency and Parallelism
   - `Async/Await` and `Futures`
   - Task Executors
   - Concurrency
4. Macros and Meta programming
   - `macro_rules!`
   - Procedural Macros
   - Derive Macros
   - Meta-programming
   - Building Domain-Specific Languages (DSL) in Rust
5. Low level and systems programming
   - [Conditional Compilation](systems-programming/conditional-compilation.rs) `cargo run --bin cc`
   - [Inline Assembly](systems-programming/inline-assembly.rs) `cargo run --bin assembly`
   - Foreign Function Interface (FFI)
   - Embedded rust and Bare-metal programming
6. Error handling and patterns
   - Advanced Error Handling
   - Custom Error Types
   - Dependency Injection patterns in rust
7. Specialized topics
   - Writing a custom allocator
   - Self-referential structs (`box`, `rc`, `Arc`)
