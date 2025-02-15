/// !to run, execute `cargo run --bin traits`
use std::{f64::consts::PI, fmt::Debug};

/// #Traits
/// -------
///
/// We can think of a trait as an abstract way to share the behavior with other
/// objects. If we came from Object Oriented programming language, we can think
/// it as an interface that can get implemented into other objects.
///
/// For example, We have different geometric shapes that has special attributes
/// such as `perimeter`, `area`, etc. In this case, we can define a trait for
/// shape that creates abstract methods `area` and `perimeter` which can then be
/// implemented by other shape objects.
///
/// Even though rust is a functional programming language, traits help reducing
/// the boundary of OOPs in rust by working in an object-oriented approach.

fn main() {
    // =========================================================================
    // The below example uses a trait `Shape` and gets implemented in different
    // structures `Rect` and `Circle`.
    println!("Traits");
    #[derive(Debug)]
    struct Rect {
        w: f64,
        h: f64,
    }
    #[derive(Debug)]
    struct Circle {
        r: f64,
    }

    // define trait Shape
    trait Shape {
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;
    }

    // =========================================================================
    // implement trait on different structs `Rect` and `Circle`
    impl Shape for Rect {
        fn area(&self) -> f64 {
            self.w * self.h
        }

        fn perimeter(&self) -> f64 {
            2f64 * (self.w + self.h)
        }
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            PI * self.r.powi(2)
        }

        fn perimeter(&self) -> f64 {
            2f64 * PI * self.r
        }
    }

    let rectangle = Rect { w: 5f64, h: 10f64 };
    let circle = Circle { r: 10f64 };

    println!("Area of rectangle is: {}", rectangle.area());
    println!("Perimeter of rectangle is: {}", rectangle.perimeter());
    println!("Area of circle is: {}", circle.area());
    println!("Perimeter of circle is: {}", rectangle.perimeter());

    // =========================================================================
    // Not only we can implement trait on a structure, we can also pass a
    // parameter as a trait. This can also be thought of a generic parameter
    // since, we can pass different type of object as a parameter that has
    // implemented that specific trait.

    fn describe_area(shape: &(impl Shape + Debug)) {
        println!("The area of {:?} is: {}", shape, shape.area());
    }

    describe_area(&rectangle);
    describe_area(&circle);

    // =========================================================================
    // Instead of passing trait as a parameter, it will be easier to read and
    // the code will be clean enough if we implement it using the trait bound
    // syntax.
    //
    // Use trait as a boundary or trait bound

    fn describe_perimeter<T: Shape + Debug>(shape: &T) {
        println!("The area of {:?} is: {}", shape, shape.perimeter())
    }
    describe_perimeter(&rectangle);
    describe_perimeter(&circle);

    // =========================================================================
    // we can even implement trait bound using `where` clause so that code will
    // be more readable.
    fn describe_area_1<T>(shape: &T)
    where
        T: Debug + Shape,
    {
        println!("The area of {:?} using `where` is: {}", shape, shape.area());
    }
    describe_area_1(&rectangle);
    describe_area_1(&circle);

    // =========================================================================
}
