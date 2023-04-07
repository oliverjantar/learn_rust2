use std::{cell::RefCell, sync::Arc, time::Duration};
struct State {
    count: usize,
}
impl State {
    fn inc(&mut self) {
        self.count += 1;
    }
}
fn main() {
    let mut state = State { count: 0 };
    let b = RefCell::new(state);
    let c = Arc::new(b);
    let c_a = c.clone();

    let c_b = c.clone();
    // let r = std::thread::spawn(move || loop {
    //     let x = (*c_a.borrow()).count;
    //     println!("{0}", x);
    //     std::thread::sleep(Duration::from_secs(1));
    // });
    // let w = std::thread::spawn(move || loop {
    //     c_b.borrow_mut().inc();
    //     std::thread::sleep(Duration::from_secs(1));
    // });
}

pub fn run() {
    println!("test");
}
