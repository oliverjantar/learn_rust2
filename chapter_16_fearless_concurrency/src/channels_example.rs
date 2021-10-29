pub fn run() {
    create_channel();
}

use std::{sync::mpsc, thread, time::Duration};

fn create_channel() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let handle = thread::spawn(move || {
        let messages = vec![
            String::from("Hi,"),
            String::from("how"),
            String::from("are"),
            String::from("you?"),
        ];

        for i in messages.into_iter() {
            tx1.send(i).unwrap_or_else(|err| {
                println!("Thread 1: Error while sending message to channel {}", err);
            });
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in values.into_iter() {
            tx.send(i.to_string()).unwrap_or_else(|err| {
                println!("Thread 2: Error {}", err);
            });
            thread::sleep(Duration::from_millis(700));
        }
    });

    // drop(rx);

    for msg in rx {
        println!("{}", msg);
    }

    handle.join().unwrap();
    handle2.join().unwrap();
}
