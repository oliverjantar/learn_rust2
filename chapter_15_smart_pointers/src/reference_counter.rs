use std::rc::Rc;

pub fn run() {
    create_list_with_shared_item();
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn create_list_with_shared_item() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("reference count to a {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("reference count to a {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("reference count to a {}", Rc::strong_count(&a));
    }
    println!("reference count to a {}", Rc::strong_count(&a));
}
