/// ! to run, execute: `cargo run --bin macro`
/// # Macro Rules
/// --------------
///
/// Macro rules are function-like statements that ends with bang or exclamation
/// sign `!` . Macro rules help us generate rust code out of raw instructions.
/// It is also useful when we want to create a domain-specific language (DSL).
///
/// As it takes raw instruction, we need to know that it accepts abstract syntax
/// tree (AST) rather than exact values. The AST that can be used inside macro
/// rules are as follows:
///
/// 1. Designators
///     - `block`       : Block Expression
///     - `expr`        : Expressions
///     - `ident`       : Identifiers
///     - `item`        : Item
///     - `literal`     : Literal Constants
///     - `pat`         : Patterns
///     - `path`        : TypePath
///     - `stmt`        : Statements
///     - `tt`          : Token Tree
///     - `ty`          : Type
///     - `vis`         : Visibility Qualifier
///
/// 2. Overload
/// 3. Repeat
///
/// Please check rustbook for more information:
/// <https://doc.rust-lang.org/rust-by-example/macros.html>
///
fn main() {
    println!("Meta Programming");
    // -------------------------------------------------------------------------
    // Example 1: add macro that accepts any number of arguments
    {
        /// This Macro rule accepts variable number of parameters and returns
        /// the sum of it
        macro_rules! add {
            ($($num: expr),*) => {   //
                // it adds all the numbers with 0 (+ [num])*
                // it adds all the numbers with 0 + [num_1] + num_2 + ...
                    0 $(+ $num)*
            };
        }

        println!("sum with macro: {}", add!(1, 2, 3, 4, 5, 6));
    }
    // -------------------------------------------------------------------------
    // Example 2: python-like  List Comprehension
    {
        macro_rules! lc {
            [($exp: expr) for $ident: ident in $iter: expr] => {
                $iter.into_iter().map(|$ident| $exp).collect()
            };

            [($exp: expr) for $ident: ident in $iter: expr; if $condition: expr] => {
                $iter
                    .into_iter()
                    .filter(|$ident| $condition)
                    .map(|$ident| $exp)
                    .collect()
            };
        }

        let squares: Vec<u32> = lc![(x*x) for x in 1..10];
        let odd_squares: Vec<u32> = lc![(x*x) for x in 1..10; if x%2!=0];

        println!("Comprehension: {:?}", squares);
        println!("Comprehension with condition: {:?}", odd_squares);
    }
    // -------------------------------------------------------------------------
}
