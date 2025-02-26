/// ! to run, execute `cargo run --bin generics`
/// # Generic Types
/// ---------------
///
/// A generic type is a placeholder type that assumes that any compatible type
/// can be accepted in the block of code. Generics are used to avoid code
/// duplication when similar operation can be done for multiple types.
///
/// For example: We can add or subtract 2 values that can be both i32 or f32.
/// If we want to create an addition function, then we can use generics to add
/// both integers and floating point values.
///
/// NOTE: Remember that when we try to do certain operations generic parameter
/// might need to specify specific property of the argument to work properly.
/// For example: addition operation should specify that the type T should
/// implement the trait std::ops::Add that outputs type T
///
/// Example, if we pass vector instead of integer, addition will not work
///
/// To know more about generics, please check:
/// https://doc.rust-lang.org/book/ch10-01-syntax.html

fn main() {
    println!("Generics Example");

    // =========================================================================
    // Basic Addition with generics
    {
        /// # Add
        /// A generic function that accepts type T that implements Add trait
        /// It means, it accepts both parameters `a` and `b` of the same type T
        /// which can be added.
        fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
            a + b
        }

        let (a, b) = (5, 10);
        println!("Adding integers ({} + {}): {}", a, b, add(a, b));

        let (a, b) = (5.5, 10.8);
        println!("Adding Floating point ({} + {}): {}", a, b, add(a, b));

        // If we uncomment the code below, it does not compile since there is no
        // Add trait implemented for the type Vector.
        // let (a, b) = (vec![1, 2], vec![3, 4]);
        // println!("Adding Floating point ({:?} + {:?}): {:?}", a, b, add(a, b));

        // Error: no implementation for `Vec<{integer}> + Vec<{integer}>`
    }

    // =========================================================================
    // We can also use Generic types with Structs
    {
        // Here, Coordinate can be both integer and floating points
        #[derive(Debug)]
        struct Coordinate<T: std::ops::Add<Output = T>> {
            x: T,
            y: T,
        }

        /// create an `add()` method implementation to the generic structure
        /// `Coordinate` that also implements same type T
        impl<T: std::ops::Add<Output = T>> Coordinate<T> {
            fn add(a: Self, b: Self) -> Self {
                Self {
                    x: a.x + b.x,
                    y: a.y + b.y,
                }
            }
        }

        let a1 = Coordinate { x: 5, y: 10 };
        let b1 = Coordinate { x: 5, y: 10 };
        println!("a1 + b1 = {:?}", Coordinate::add(a1, b1));
        let a2 = Coordinate { x: 5.5, y: 10.2 };
        let b2 = Coordinate { x: 5.5, y: 10.2 };
        println!("a2 + b2 = {:?}", Coordinate::add(a2, b2));
    }
    // =========================================================================
}
