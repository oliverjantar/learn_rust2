use std::{borrow, cell::RefCell, rc::Rc};

pub fn run() {
    mutable_tree_example();
}

#[derive(Debug)]
enum List {
    Node(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Nil, Node};

fn mutable_tree_example() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Node(Rc::clone(&value), Rc::new(Nil)));

    let b = Node(Rc::new(RefCell::new(7)), Rc::clone(&a));

    let c = Rc::new(Node(Rc::new(RefCell::new(3)), Rc::clone(&a)));

    let d = Node(Rc::new(RefCell::new(9)), Rc::clone(&c));

    {
        *value.borrow_mut() += 10;
    }

    {
        *value.borrow_mut() += 10;
    }

    match d {
        Node(ref x, _) => {
            println!("d is list");
            *x.borrow_mut() += 15;
            println!("d is {:?}", d);
        }
        Nil => println!("d is nil"),
    }

    println!("a is {:?}", a);
    println!("b is {:?}", b);
    println!("c is {:?}", c);
}
