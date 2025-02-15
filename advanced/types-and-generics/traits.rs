/// !to run, execute `cargo run --bin traits`
use std::f64::consts::PI;

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
    println!("Traits");

    // using traits in different shapes
    {
        struct Rect {
            w: f64,
            h: f64,
        }

        struct Circle {
            r: f64,
        }

        // define trait Shape
        trait Shape {
            fn area(&self) -> f64;
            fn perimeter(&self) -> f64;
        }

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
    }
}
