# Rust Raid

## Algorithms

This section contains different algorithms for solving basic problems. As the
challenge is intended to teach algorithm themselves, we will not be using any
built-in methods such as `sort()`, `index()`, etc.

List Of Algorithms are as follows:

### searching

1. [Linear Searching](src/searching/linear_search.rs) `cargo run --bin linear_search`
2. [Binary Searching](src/searching/binary_search.rs) `cargo run --bin binary_search`
3. [Depth First Search (DFS)]
4. [Breadth First Search (BFS)]

### sorting

1. [bubble sort](src/sorting/bubble_sort.rs) `cargo run --bin bubble_sort`
2. [selection sort](src/sorting/selection_sort.rs) `cargo run --bin selection_sort`
3. [insertion sort](src/sorting/insertion_sort.rs) `cargo run --bin insertion_sort`
4. [quick sort](src/sorting/quick_sort.rs) `cargo run --bin quick_sort`
5. [Merge sort](src/sorting/merge_sort.rs) `cargo run --bin merge_sort`
6. [heap Sort]
7. [Counting Sort]
8. [Radix Sort]

### Greedy and graph algorithms

1. [Activity Selection]
2. [Huffman Coding](src/greedy/huffman_coding.rs) `cargo run --bin huffman`
3. [Krushkal's algorithm](src/greedy/kruskal.rs) `cargo run --bin kruskal`
4. [Prim's Algorithm]
5. [Dijkstra's Algorithm]
6. [Bellman-Ford Algorithm]
7. [Floyd-Warshall Algorithm]
8. [Topological Sort]
9. [A* Search Algorithm]

### Miscellaneous Algorithms

1. Luhn's algorithm to validate credit card number.

   ```shell
   cargo run --bin luhn
   cargo test --bin luhn
   ```
