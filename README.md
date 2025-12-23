# Rust Raid

![GitHub commit activity](https://img.shields.io/github/commit-activity/t/ghimiresdp/rust-raid)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/ghimiresdp/rust-raid/rust.yml)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/ghimiresdp/rust-raid)

> [!IMPORTANT]
> **DISCLAIMER !!**
>
> This project is not affiliated with, endorsed by, or sponsored by the Rust
> Foundation or the Rust programming language project. "Rust" and the Rust logo
> are trademarks of the Rust Foundation.
> For details on trademark usage, please refer to the Rust Foundation Trademark
> Policy.

[Rust Raid](https://github.com/ghimiresdp/rust-raid) is a repository for Rust
learners and coding challenge seekers.

> [!NOTE]
> In some linux distributions (especially WSL), the following packages
> might need to be installed to run some binaries that require network request.
>
> 1. **`pkg-config`**
> 2. **`libssl-dev`**
>
> Example commands for installing in ubuntu
>
> ```bash
> sudo apt install pkgconf
> sudo apt install libssl-dev
> ```
>
> You can check similar commands for other distributions

**The repository contains the following**:

- `Algorithms` that can be used to solve various problems.
- `Examples` of different `design patterns` and `data structures`.
- `solutions` to diverse challenges categorized by different topics(workspaces).
- `advanced concepts` that include memory management, multiprocessing, etc.
- `Demo projects` to polish your skills to the depth.

Each workspace contains multiple binaries so that it will be easier to run
a specific problem by selecting binaries.

You can run the `cargo run --bin <binary_name>` to run binaries. To run all test cases,
you can run `cargo test` command, or to run specific test, you can run
`cargo test --bin <binary_name>`

```bash
# Example: running binary for huffman encoding
cargo run --bin huffman
cargo test --bin huffman
```

> [!NOTE]
> _Topics that do not contain hyperlinks are work in progress and will be_
> _updated once the solution gets completed._
>
> _You can also create a PR with the solution/enhancement to each topic._

## [1. Data Structures](data-structures/)

1. **Arrays**
   - [Find the missing number](data-structures/src/missing-number.rs) `cargo run --bin missing-number`
   - [Find the length of the longest subarray with sum K](data-structures/src/longest-subarray.rs) `cargo run --bin longest-subarray`
2. **Singly Linked Lists**
   - [Add two numbers represented by linked list](data-structures/src/linked-list-add.rs) `cargo run --bin linked-list-add`
3. [**Doubly Linked Lists**](data-structures/src/doubly_linked_list.rs) `cargo run --bin doubly_linked_list`
4. [**Stacks**](data-structures/src/stack.rs) `cargo run --bin stack`
5. [**Queues**](data-structures/src/queue.rs) `cargo run --bin queue`
6. [**Binary Trees**](data-structures/src/binary_tree.rs) `cargo run --bin binary_tree`
7. [**Trie**](data-structures/src/trie.rs) `cargo run --bin trie`

## [2. Algorithms](./algorithms/README.md)

### [2.1. Searching](algorithms/src/searching/)

1. [Linear Searching](algorithms/src/searching/linear_search.rs) `cargo run --bin linear_search`
2. [Binary Searching](algorithms/src/searching/binary_search.rs) `cargo run --bin binary_search`
3. [Depth First Search (DFS)]
4. [Breadth First Search (BFS)]

### [2.2. Sorting](algorithms/src/sorting/)

1. [bubble sort](algorithms/src/sorting/bubble_sort.rs) `cargo run --bin bubble_sort`
2. [selection sort](algorithms/src/sorting/selection_sort.rs) `cargo run --bin selection_sort`
3. [insertion sort](algorithms/src/sorting/insertion_sort.rs) `cargo run --bin insertion_sort`
4. [quick sort](algorithms/src/sorting/quick_sort.rs) `cargo run --bin quick_sort`
5. [Merge sort](algorithms/src/sorting/merge_sort.rs) `cargo run --bin merge_sort`
6. [heap Sort]
7. [Counting Sort]
8. [Radix Sort]

### [2.3 Greedy and Graph Algorithms](algorithms/src/greedy/)

1. [Activity Selection]
2. [Huffman Coding](algorithms/src/greedy/huffman_coding.rs) `cargo run --bin huffman`
3. [Krushkal's algorithm](algorithms/src/greedy/kruskal.rs) `cargo run --bin kruskal`
4. [Prim's Algorithm]
5. [Dijkstra's Algorithm]
6. [Bellman-Ford Algorithm]
7. [Floyd-Warshall Algorithm]
8. [Topological Sort]
9. [A* Search Algorithm]

## [3. Design Patterns](./design-patterns/README.md)

1. [Singleton Pattern](design-patterns/src/singleton.rs) `cargo run --bin singleton`
2. [Factory Pattern](design-patterns/src/factory.rs) `cargo run --bin factory`
3. [Builder Pattern](design-patterns/src/builder.rs) `cargo run --bin builder`
4. [Decorator Pattern](design-patterns/src/decorator.rs) `cargo run --bin decorator`
5. [Observer Pattern](design-patterns/src/observer.rs) `cargo run --bin observer`
6. [Strategy Pattern](design-patterns/src/strategy.rs) `cargo run --bin strategy`
7. [Command Pattern](design-patterns/src/command.rs) `cargo run --bin command`
8. [Adapter Pattern](design-patterns/src/adapter.rs) `cargo run --bin adapter`

## [4. Problem Solving](problem-solving/README.md)

### [4.1. Basic Problems](problem-solving/src/basic/)

1. [Practical Number](problem-solving/src/basic/practical_number.rs) `cargo run --bin practical_number`
2. [Greatest Common Divisor](problem-solving/src/basic/gcd.rs) `cargo run --bin gcd`
3. [Median](problem-solving/src/basic/median.rs) `cargo run --bin median`
4. [Reverse digits of the integer](problem-solving/src/basic/reverse_integer.rs) `cargo run --bin reverse_integer`
5. [List Comprehension](problem-solving/src/basic/comprehension.rs) `cargo run --bin comprehension`
6. [Linear Regression Model](problem-solving/src/basic/linear_regression.rs) `cargo run --bin linear_regression`
7. [Matrix Multiplication Model](problem-solving/src/basic/matrix_multiplication.rs) `cargo run --bin matrix_multiplication`
8. [Color Converter](problem-solving/src/basic/color-converter.rs) `cargo run --bin color-converter`

### [4.2. Dynamic Programming](problem-solving/src/dp/)

1. [List group by consecutive numbers](problem-solving/src/dp/consecutive_groups.rs) `cargo run --bin consecutive_groups`
2. [Find the length of the longest substring with maximum 2 repetition](problem-solving/src/dp/repeat.rs)`cargo run --bin repeat`
3. [Find the index of two numbers in an array whose sum equals to the provided target](problem-solving/src/dp/two_sum.rs) `cargo run --bin two_sum`
4. [Minimize the Sum from an array](problem-solving/src/dp/minimize_sum.rs) `cargo run --bin minimize_sum`
5. [Fibonacci Series](problem-solving/src/dp/fibonacci.rs) `cargo run --bin fibonacci`
6. [Longest Common Subsequence](problem-solving/src/dp/longest_common_subsequence.rs) `cargo run --bin lcs`
7. [Coin Change Problem](problem-solving/src/dp/coin_change.rs) `cargo run --bin coin_change`
8. [Palindrome Partition](problem-solving/src/dp/palindrome_partition.rs) `cargo run --bin palindrome_partition`
9. [Find Nth distinct number](problem-solving/src/dp/nth-distinct-number.rs) `cargo run --bin nth-distinct-number`

## [5. Advanced Concepts](advanced/README.md)

### 5.1. Memory Management

- [Ownership, borrowing, and Lifetimes](advanced/memory-management/src/ownership.rs) `cargo run --bin ownership`
- [Unsafe Rust](advanced/memory-management/src/unsafe.rs)

### 5.2. Type System and Generics

- [Generic Types](advanced/types-and-generics/src/generics.rs) `cargo run --bin generics`
- [Trait Objects and Dynamic Dispatch](advanced/types-and-generics/src/traits.rs) `cargo run --bin traits`
- Associated types and Generic Type parameters
- Lifetime Sub-typing

### 5.3. Concurrency and Parallelism

- [`Async/Await` and `Futures`](advanced/concurrent-programming/src/threading.rs) `cargo run --bin threading`
- [Thread Spawning](advanced/concurrent-programming/src/spawning.rs) `cargo run --bin spawning`

### 5.4. Macros and Meta programming

- [`macro_rules!`](advanced/meta-programming/src/macro-rules.rs) `cargo run --bin macro`
- [Derive Macros](advanced/meta-programming/src/derive-macro.rs) `cargo run --bin derive`
- [Building Domain-Specific Languages (DSL)](advanced/meta-programming/src/dsl.rs) `cargo run --bin dsl`

### 5.5. Low-level and systems programming

- [Conditional Compilation](advanced/systems-programming/src/conditional-compilation.rs) `cargo run --bin cc`
- [Inline Assembly](advanced/systems-programming/src/inline-assembly.rs) `cargo run --bin assembly`
- Foreign Function Interface (FFI)
- Embedded rust and Bare-metal programming

### 5.6. Error handling and patterns

- [Unrecoverable error and `panic!` macro](advanced/error-handling/src/panic.rs) `cargo run --bin panic`
- [Recoverable error and `Result` enum](advanced/error-handling/src/result.rs) `cargo run --bin result`
- Advanced Error Handling
  - [Propagating Errors with `?` operator](advanced/error-handling/src/propagation.rs) `cargo run --bin propagation`
  - [Custom Errors](advanced/error-handling/src/custom-error.rs) `cargo run --bin custom-error`
- Dependency Injection patterns in rust

### [5.7. Operator Overloading](advanced/operator-overloading/src/main.rs) `cargo run --bin operator-overloading`

- Example 1: Operator Overloading in structs (overloading `+` and `-` operators)
- Example 2: Matrix Multiplication ( overloading `*` operator)
- Example 3: Scalar Multiplication (operator overloading with heterogeneous data type)

### 5.8. Smart Pointers

- [Box Pointers](advanced/smart-pointers/src/box.rs) `cargo run --bin box`
- [Reference Counters (`Rc`)](advanced/smart-pointers/src/rc.rs) `cargo run --bin rc`
- [`RefCell`](advanced/smart-pointers/src/refcell.rs)`cargo run --bin refcell`
- [`Arc Mutex`](advanced/smart-pointers/src/arc-mutex.rs)`cargo run --bin arc-mutex`

### 5.9. Specialized topics

- Writing a custom allocator
- Self-referential structs (`box`, `rc`, `Arc`)

---

## [6. Projects](./projects/)

### [6.1. `pandas` - pandas-like mini-dataframe container project](projects/pandas/README.md)

Command:

```sh
cargo run --bin pandas
```

![pandas](res/images/pandas.png)

---

### [6.2. `ruscrypt` - A basic encryption mini-project](projects/ruscrypt/README.md)

Command:

```sh
cargo run --bin ruscrypt
```

---

### [6.3. Basic Password `vault`](projects/vault/README.md)

Command:

```sh
cargo run --bin vault
```

---

### [6.4. A Basic `TODO` Web Application mini project](projects/todo-web/README.md)

Command:

```sh
cargo run --bin todo
```

and browse: <http://localhost:8080>

![todo](res/images/todo.png)

---

### [6.5. `cget` - a wget-like file downloader utility with multithreading](projects/cget/Cargo.toml)

Command: `cargo run --bin cget <DOWNLOAD_URL>`

Example:

```shell
cargo run --bin cget https://mirrors.saswata.cc/kde/stable/krita/5.2.14/krita-x64-5.2.14-setup.exe
```

![cget](res/images/cget.png)

## Running binaries

To run any binary, you can run the command `cargo run --bin <bin_name>`

Example:

```shell
cargo run --bin practical_number
```

> [!NOTE]
> Binary names might not always be the name of the file. Sometimes, a
> shorter version of the solution name is used to make easier to type. You can
> see the name of binary in the respective `README.md` file or the `docstring`
> of the respective solution.
> Example:
>
> - The binary for `huffman_coding.rs` is just `huffman`.

## Testing

There are test cases for each function/challenge that will be beneficial
for you to learn testing as well as test programs for errors.

To test programs, you can run `cargo test` command.

Example:

```shell
cargo test

# alternatively, to test individual binary, you can run
cargo test --bin your_program_name
```
