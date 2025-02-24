/// ! to run, execute: `cargo run --bin dsl`
/// Domain Specific Language (DSL)
/// ------------------------------
///
/// DSL is a  custom programming language that can be evaluated on specific type
/// of data or specific domain.
///
/// An example of DSL is SQL. SQL works for a list that contains similar type of
/// data. Not only RDBMS supports SQL, but any iterable that behaves like RDBMS
/// can be queried using SQL.
///
/// The below will use custom SQL-like query to list data from iterable that is
/// capable of querying, filtering, and ordering the data.
///
/// We use macro_rules! to create a SQL-like dsl.

macro_rules! sql {
    // SELECT x from <data> WHERE <condition>
    (SELECT $var: ident FROM $iter: expr, WHERE $cond: expr) => {
        $iter.into_iter().filter(|&$var| $cond).collect()
    };

    // SELECT (ASC x as <type>) from <data> WHERE <condition>
    (SELECT (ASC $var: ident AS $t:ty) FROM $iter: expr, WHERE $cond: expr) => {{
        let mut x: $t = $iter.into_iter().filter(|&$var| $cond).collect();
        x.sort();
        x
    }};

    // SELECT ( DESC x as <type>) from <data> WHERE <condition>
    (SELECT (DESC $var: ident AS $t:ty) FROM $iter: expr, WHERE $cond: expr) => {{
        let mut x: $t = $iter.into_iter().filter(|&$var| $cond).collect();
        x.sort_by(|a, b| b.cmp(a));
        x
    }};
}
fn main() {
    let result: Vec<u32> = sql!(SELECT x FROM 1..10, WHERE x>=5);
    println!("The SQL output is: {:?}", result);

    let result = sql!(SELECT (ASC x AS Vec<u32>) FROM 1..10, WHERE x>=5);
    println!("SQL output [ASC]: {:?}", result);

    let result = sql!(SELECT (DESC x AS Vec<u32>) FROM 1..10, WHERE x>=5);
    println!("SQL output [DESC]: {:?}", result);
}
