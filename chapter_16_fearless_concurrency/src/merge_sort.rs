use core::num;
use std::array;
use std::sync::mpsc;
use std::thread;

pub fn run() {
    let mut input = [4, 32, 1, 5, 234, 12, 53, 8, 93, 52, 16, 11, 7, 20];

    // x = vec![1, 5, 3, 2, 3, 5, 6, 4, 3, 1, 8, 4];

    let MAX_THREADS: u8 = 8;

    let length = input.len();

    let num_threads = match length {
        l if l < 4 => 1,
        l if l >= 4 && l < 7 => 2,
        l if l >= 7 && l < 10 => 3,
        l if l >= 10 => 4,
        _ => panic!(
            "error when selecting number of threads for length {}",
            length
        ),
    };

    // let chunk = match array_length % num_threads == 0 {
    //     true => array_length / num_threads,
    //     false => (array_length / num_threads) + 1,
    // };

    let chunk = length / num_threads;
    let mut reminder = length % num_threads;

    println!("array: {:?}", input);
    println!("length: {}", length);
    println!("num_threads: {}", num_threads);
    println!("chunk: {}", chunk);
    println!("reminder: {}", reminder);

    let mut item_index = 0;
    let mut handles = vec![];

    let (tx, rx) = mpsc::channel();

    for i in 1..num_threads + 1 {
        let thread_chunk = match reminder > 0 {
            true => {
                reminder -= 1;
                chunk + 1
            }
            false => chunk,
        };

        let mut array_chunk: Vec<i32> = vec![0; thread_chunk];
        let chunk_slice = &input[item_index..item_index + thread_chunk];

        println!("array_chunk {:?}", array_chunk);
        println!("chunk_slice {:?}", chunk_slice);

        array_chunk.clone_from_slice(chunk_slice);

        item_index = item_index + thread_chunk;

        println!(
            "thread: {}, chunk: {}, array: {:?}",
            i, thread_chunk, array_chunk
        );

        let _tx = tx.clone();

        handles.push(thread::spawn(move || {
            let sorted = sort(array_chunk);

            println!("sending data from thread {}", i);
            _tx.send(sorted)
        }));
    }

    drop(tx);
    let mut sorted: Vec<i32> = Vec::new();
    for data in rx {
        sorted = merge(sorted, data);
        println!("sorted data {:?}", sorted);
    }

    for t in handles {
        t.join().unwrap();
    }
    println!("result is {:?}", sorted);
}

fn sort(items: Vec<i32>) -> Vec<i32> {
    if items.len() == 1 {
        return items;
    }

    let middle = items.len() / 2;

    let left = &items[..middle];
    let right = &items[middle..];

    println!("left {:?}, right {:?}", left, right);

    let left_sorted = sort(left.to_vec());
    let right_sorted = sort(right.to_vec());
    merge(left_sorted, right_sorted)
}

fn merge(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
    left.append(&mut right);
    left.sort();
    left
}
