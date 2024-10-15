use std::sync::{Arc, RwLock};
use std::thread;

fn fun() {
    let mut read_lock = data_clone.read().unwrap(); // Acquire read lock
    read_lock.push(100);
    println!("Read: {:?}", *read_lock);
}
fn main() {
    // Create a new RwLock protecting a Vec of integers
    let data = Arc::new(RwLock::new(Vec::new()));

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    // Spawn writer threads
    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write_lock = data_clone.write().unwrap(); // Acquire write lock
            write_lock.push(i);
            println!("Written: {}", i);
        });
        handles.push(handle);
    }

    // Spawn reader threads
    for _ in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            func();
        });
        handles.push(handle);
    }

        
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
