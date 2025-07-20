#![allow(dead_code)]

use std::rc::Rc;

/// # Reference Counted Smart Pointer `Rc<T>`
///
/// Rc is a smart pointer that keeps track of multiple owners. If we want
/// multiple owners for the single resource, then Rc will help us keeping track
/// of number of references to a value so that when number of references become
/// zero, it can be cleaned up.
///
/// Even though rustlang does not contain Garbage Collector(GC), Rc works
/// somewhat similar to GC with a bit different philosophy.
///
/// In case of Rc, the value is dropped immediately as the RC has 0 number of
/// references, whereas GC is not as deterministic as Rc.
///
/// Also, Rc implements traits with `drop` method which allows us to manually
/// drop the value when not needed, however in case of GC, manual drop is not
/// possible.
///
/// Ref: https://doc.rust-lang.org/book/ch15-04-rc.html
fn main() {
    println!("Reference Counters");

    // -------------------------------------------------------------------------
    {
        // A simple example of Rc
        let a = Rc::new(String::from("Hello Rc"));
        println!("Reference count = {}", Rc::strong_count(&a)); // 1
        {
            let b = Rc::clone(&a);
            println!("Reference count = {}", Rc::strong_count(&a)); // 2
            println!("b = {}", b);
        } // reference to b is dropped so RC is 1 again
        println!("Reference count = {}", Rc::strong_count(&a)); //1
        println!("a = {}", a);
        drop(a); // manual dropping of the value

        // line below gives error when uncommented since RC is 0 here.
        // println!("a = {}", a);
    }
    // -------------------------------------------------------------------------
    {
        /// Using Rc in a Graph data type (multiple ownership)
        /// example:
        /// A(1)       B(2) (Both A and B references AB as the child)
        ///     \    /
        ///     AB(12)

        #[derive(Debug)]
        struct Node<T> {
            value: T,
            children: Vec<Rc<Node<T>>>,
        }

        let ab = Rc::new(Node {
            value: 12,
            children: vec![],
        });
        print!("Reference is set, ");
        println!("Reference Count of AB: {}", Rc::strong_count(&ab)); // 1

        let a = Node {
            value: 10,
            children: vec![Rc::clone(&ab)],
        };
        print!("Reference is added, ");
        println!("Reference Count of AB: {}", Rc::strong_count(&ab)); // 2

        let b = Node {
            value: 10,
            children: vec![Rc::clone(&ab)],
        };
        print!("Reference is added, ");
        println!("Reference Count of AB: {}", Rc::strong_count(&ab)); // 3

        println!("Node A: {:?}", a);
        println!("Node B: {:?}", b);

        drop(a);
        print!("A is dropped, ");
        println!("Reference Count of AB: {}", Rc::strong_count(&ab)); //2

        drop(b);
        print!("B is dropped,");
        println!("Reference Count of AB: {}", Rc::strong_count(&ab)); //1
    }
}
