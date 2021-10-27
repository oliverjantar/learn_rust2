pub fn run() {
    //spawn_thread_dont_wait();
    //spawn_thread_and_wait();
    move_values_to_thread();
}

use std::thread;
use std::time::Duration;

fn spawn_thread_dont_wait() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn spawn_thread_and_wait() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn move_values_to_thread() {
    let v = vec![1, 2, 3, 4];

    let handle = thread::spawn(move || {
        for i in v.iter() {
            println!("hi from spawned thread {}", i);
        }
    });

    handle.join().unwrap();
}
