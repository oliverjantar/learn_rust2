pub fn run() {
    //mutex_example();
    shared_value();
}

use std::sync::{Arc, Mutex};
use std::{thread, vec};

fn mutex_example() {
    let x = 5;
    let m = Mutex::new(x);

    {
        let mut num = m.lock().unwrap();
        *num += 1;
    }
    println!("m = {:?}", m);
}

fn shared_value() {
    let m = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let cloned_m = m.clone();
        handles.push(thread::spawn(move || {
            let mut c = cloned_m.lock().unwrap();
            *c += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("result {:?}", m);
}
