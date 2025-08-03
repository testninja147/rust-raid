#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// # Refcell
///
/// Regular pointers often suffer from one specific issue of the rust design
/// pattern. a data can either have a single mutable reference or multiple
/// immutable references, but not both.
///
/// Refcell is a smart pointer that came to resolve the issue of the above issue
/// which implements interior mutability pattern. We can mutate data wrapped by
/// the refcell even it is immutable.
///
/// That means, the rust's borrow checker will not be able to know the behavior
/// of the block during compilation, and if we are sure it is going to follow
/// the borrow checker rules will get followed at runtime, we can use refcell.
///
///
/// Ref: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
fn main() {
    println!("Refcell");

    // -------------------------------------------------------------------------
    {
        // Example 1: mutating immutable value using refcell

        // let x = Vec::new();
        // let y = &mut x;  // not allowed since x is immutable

        // to overcome the issue, we wrap the vector to the refcell so that it
        // features interior mutability even for immutable objects
        let x = RefCell::new(Vec::new());
        x.borrow_mut().push(5u32);
        println!("{:?}", x);
        //Output: RefCell { value: [5] }
    }

    // -------------------------------------------------------------------------
    {
        // Example 2: mutable references usecase

        #[derive(Debug)]
        struct HtmlElement {
            props: RefCell<HashMap<String, String>>,
        }

        impl HtmlElement {
            fn new() -> Self {
                Self {
                    props: RefCell::new(HashMap::new()),
                }
            }
        }

        let a = HtmlElement::new();

        // ---------------------------------------------------------------------
        // the code below compiles without any errors but panics at runtime
        // since there are multiple mutable references at the same time.

        // let mut prop_borrow_1 = a.props.borrow_mut();
        // let mut prop_borrow_2 = a.props.borrow_mut();
        // prop_borrow_1.insert("href".to_owned(), "https://rust-lang.org".to_owned());
        // prop_borrow_2.insert("target".to_owned(), "_blank".to_owned());

        // ---------------------------------------------------------------------
        // so to avoid the above issue, we have to either borrow the data in its
        // own lifetime and borrow the second one only after first borrow has
        // been cleaned up.

        {
            let mut prop_borrow_1 = a.props.borrow_mut();
            prop_borrow_1.insert("href".to_owned(), "https://rust-lang.org".to_owned());
        }
        {
            let mut prop_borrow_2 = a.props.borrow_mut();
            prop_borrow_2.insert("target".to_owned(), "_blank".to_owned());
        }

        println!("{:?}", a.props.borrow());
        // {"href": "https://rust-lang.org", "target": "_blank"}
    }

    // -------------------------------------------------------------------------
    // Example 3: better implementation with RC + RefCell
    {
        #[derive(Debug)]
        struct HtmlElement {
            props: Rc<RefCell<HashMap<String, String>>>, // Rc allows multiple owners
        }

        impl HtmlElement {
            fn new() -> Self {
                Self {
                    props: Rc::new(RefCell::new(HashMap::new())),
                }
            }
        }

        let a = HtmlElement::new();
        let props_clone1 = Rc::clone(&a.props);
        let props_clone2 = Rc::clone(&a.props);

        // First owner mutates
        {
            let mut props = a.props.borrow_mut();
            props.insert("href".to_owned(), "https://rust-lang.org".to_owned());
        }

        // Second owner mutates using props_clone1
        {
            let mut props = props_clone1.borrow_mut();
            props.insert("target".to_owned(), "_blank".to_owned());
        }

        // Third owner mutates using props_clone2
        {
            let mut props = props_clone2.borrow_mut();
            props.insert("class".to_owned(), "ext-link".to_owned());
        }

        // All owners see the same data
        println!("a: {:?}", a.props.borrow());
        println!("props_clone1: {:?}", props_clone1.borrow());
        println!("props_clone2: {:?}", props_clone2.borrow());
        // Output:
        // a: {"href": "https://rust-lang.org", "target": "_blank", "class": "ext-link"}
        // props_clone1: {"href": "https://rust-lang.org", "target": "_blank", "class": "ext-link"}
        // props_clone2: {"href": "https://rust-lang.org", "target": "_blank", "class": "ext-link"}

        println!(
            "Number of references of a.props: {:?}",
            Rc::strong_count(&a.props)
        );
        // Output:
        // Number of references of a.props: 3
    }
}
