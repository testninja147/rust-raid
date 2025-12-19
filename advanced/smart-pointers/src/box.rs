/// # Smart Pointers
///
/// Smart Pointers are data structures with the behavior of pointers with additional features.
///
/// Smart pointers are used when we need multiple owners to the same resource. They are implemented
/// using structs. Some examples of smart pointers are: Box, Rc, Ref, etc.
///
/// REF: <https://doc.rust-lang.org/book/ch15-00-smart-pointers.html>
///
/// ## Box pointer
///
/// Box pointer (`Box<T>`) is a pointer that stores data on the Heap. They are generally used when we do not
/// know the exact size of the value at the compile time or when we have to transfer the ownership
/// of a large data.
///
/// We generally initialize a Box pointer with the `Box::new()` method.
///
/// A good usecase of a box pointer is linked list. Since a linked list may have
/// an infinite size, we cannot store the linked list directly without using
/// box pointer.
/// Ref: <https://doc.rust-lang.org/book/ch15-01-box.html>
fn main() {
    println!("Box pointers");

    #[derive(Debug)]
    struct LinkedList<T> {
        head: Node<T>,
    }

    // the code below gives error since the Node has infinite size
    // or the size can not be evaluated at runtime

    // struct Node<T> {
    //     value: T,
    //     next: Option<Node<T>>,
    // }

    // to overcome the issue, we wrap the Node with the Box pointer so that it
    // will store the content in the heap.
    #[derive(Debug)]
    struct Node<T> {
        value: T,
        next: Option<Box<Node<T>>>,
    }

    let linked_list = LinkedList {
        head: Node {
            value: 5,
            next: Some(Box::new(Node {
                value: 6,
                next: None,
            })),
        },
    };

    let mut node = &linked_list.head;
    loop {
        println!("The value of the node is: {}", node.value);
        if let Some(next) = &node.next {
            node = &next;
        } else {
            break;
        }
    }
}
