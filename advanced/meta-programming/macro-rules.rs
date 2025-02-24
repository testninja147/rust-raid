/// ! to run, execute: `cargo run --bin macro`
/// # Macro Rules
/// --------------
///
/// Macro rules are function-like statements that ends with bang or exclamation
/// sign `!` . Macro rules help us generate rust code out of raw instructions.
/// It is also useful when we want to create a domain-specific language (DSL).
///
/// As ot takes raw instruction, we need to know that it accepts abstract syntax
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
/// https://doc.rust-lang.org/rust-by-example/macros.html
///
fn main() {
    println!("Meta Programming");
    // -------------------------------------------------------------------------
    // Example 1: add macro that accepts any number of arguments
    {
        /// This Macro rule accepts variable number of parameters and returns
        /// the sum of it
        macro_rules! add {
            ($($number: expr),+) => {
                {
                    0 $(+ $number)*
                }
            };
        }

        println!("{}", add!(1, 2, 3, 4, 5, 6));
    }
    // -------------------------------------------------------------------------
}
