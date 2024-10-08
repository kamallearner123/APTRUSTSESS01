use std::sync::{Arc, Mutex};
use std::thread;



fn main_1() {
    // Create a new Arc to share ownership of the Mutex-protected counter.
    let counter = Arc::new(Mutex::new(String::from("Hello thread")));

    let mut handles = vec![];

    // Create multiple threads, all sharing the same counter.
    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Clone the Arc to share with the thread.

        let handle = thread::spawn(move || {
            // Lock the Mutex before accessing the counter.
            let mut num = counter.lock().unwrap();
            num.push_str("..."); // Increment the counter.
        });

        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    

    // Print the final counter value.
    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    main_1();
}
