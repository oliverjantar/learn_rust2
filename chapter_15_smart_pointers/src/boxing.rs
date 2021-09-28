use std::ops::{Deref, DerefMut};

pub fn run() {
    simple_init();
    cons();
    deref_simple();
    deref_simple2();
    my_box();
    deref_coercion();
    deref_mut_test();
    test_drop();
}

fn simple_init() {
    let b = Box::new(5);
    println!("b is {}", b);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn cons() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
}

fn deref_simple() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn deref_simple2() {
    let mut x = 5;
    let y = Box::new(x);

    x = 6;

    let z = x;

    x = 8;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(item: T) -> MyBox<T> {
        MyBox(item)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn my_box() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn deref_coercion() {
    hello("Test");
    let x = MyBox::new(String::from("Test2"));
    hello(&x);
}

fn hello(name: &str) {
    println!("Hello {}", name);
}

fn change_ref(value: &mut String) {
    value.push_str(" test");
}

fn deref_mut_test() {
    let mut x = String::from("This is a");
    change_ref(&mut x);
    println!("{}", x);

    let mut y = MyBox::new(String::from("This is another"));
    change_ref(&mut y);
    println!("{}", *y);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping custom smart pointer with data {}", self.data);
    }
}

fn test_drop() {
    println!("starting scope");
    {
        println!("inner scope");
        let x = CustomSmartPointer {
            data: String::from("test data"),
        };
        drop(x);
        println!("leaving inner scope");
    }
    println!("leaving drop fn");
}
