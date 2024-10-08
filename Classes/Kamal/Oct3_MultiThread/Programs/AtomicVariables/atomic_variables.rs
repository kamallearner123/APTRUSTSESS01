use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    // Create an atomic variable initialized to 0
    let counter = Arc::new(AtomicUsize::new(0));
    
    // Create a vector to hold the thread handles
    let mut handles = vec![];

    // Spawn 10 threads
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // Increment the atomic counter by 1
            for _ in 0..1000 {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    println!("Final counter: {}", counter.load(Ordering::SeqCst));
}
