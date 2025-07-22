use std::{
    cell::RefCell,
    fmt::Display,
    ops::Deref,
    rc::{Rc, Weak},
};

// Box pointer
#[derive(Debug)]
struct MyBox<T: Display>(T);

impl<T: Display> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

// Rc Pointer
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    ConsRef(Rc<RefCell<i32>>, Rc<List>),
    Bx(i32, Box<List>),
    Nil,
}
use crate::List::{Bx, Cons, ConsRef, Nil};

// Ref cycles and weak pointers
struct Node {
    name: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(name: &str) -> Node {
        Node {
            name: name.to_string(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }
}

fn main() {
    // Box pointer
    let nu = MyBox(5);
    let nu2 = &nu;
    //println!("{:#?}", nu2);
    drop(nu); // manually dropping this  smart pointer

    let mut a = Box::new(25);
    let b = *a;

    *a = *a + 1;
    //println!("a: {a}");
    //println!("b: {b}");

    // Rc Pointer

    // let a = Rc::new(
    //     Cons(1,
    //         Rc::new(Cons(2, Rc::new(Nil)))));

    let a = Rc::new(
        // As long the value was wrapped by Rc::new it works.
        Bx(1, Box::new(Cons(2, Rc::new(Nil)))),
    );

    let b = Cons(11, Rc::clone(&a));
    println!("Count of ref to a: {}", Rc::strong_count(&a));
    {
        let c = Cons(22, Rc::clone(&a));
        println!("Count of ref to a: {}", Rc::strong_count(&a));
    }

    // println!("{:#?}", a);
    println!("Count of ref to a: {}", Rc::strong_count(&a));

    // Refcell Pointer
    let value = Rc::new(RefCell::new(123));
    // We wrap like this because rc doesnt let us pass a mut ref. But once we deref, we can get access to runtime mutable ref from refcell.
    // Else rc would want us to either clone(it wont be mut on same value) or it will move the value in which case we will lose the "value" ref that we currently have.
    let aa = Rc::new(ConsRef(Rc::clone(&value), Rc::new(Nil)));

    let bb = Cons(111, Rc::clone(&aa));
    let cc = Cons(222, Rc::clone(&aa));

    println!("Count of ref to aa: {}", Rc::strong_count(&aa));

    println!("{:#?}", bb);
    *value.borrow_mut() = 25;
    println!("{:#?}", bb);

    //Weak pointers - move to end later
    let parent = Rc::new(Node::new("new"));
    let child = Rc::new(Node::new("child"));

    parent.children.borrow_mut().push(Rc::clone(&child));
    println!("After adding child");
    println!(
        "Parent Weak: {} Strong: {}",
        Rc::weak_count(&parent),
        Rc::strong_count(&parent)
    );
    println!(
        "Child Weak: {} Strong: {}",
        Rc::weak_count(&child),
        Rc::strong_count(&child)
    );

    *child.parent.borrow_mut() = Rc::downgrade(&parent);
    println!("After adding weak ref");
    println!(
        "Parent Weak: {} Strong: {}",
        Rc::weak_count(&parent),
        Rc::strong_count(&parent)
    );
    println!(
        "Child Weak: {} Strong: {}",
        Rc::weak_count(&child),
        Rc::strong_count(&child)
    );
}
