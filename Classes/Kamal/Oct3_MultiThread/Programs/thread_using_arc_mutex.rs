use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create some mutable data (a vector of integers)
    let data = vec![1, 2, 3, 4, 5];

    // Wrap the data in Arc and Mutex to allow shared mutable ownership among threads
    let shared_data = Arc::new(Mutex::new(data));

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    for i in 0..5 {
        // Clone the Arc to get a new reference for each thread
        let data_clone = Arc::clone(&shared_data);
        
        // Spawn a new thread
        let handle = thread::spawn(move || {
            // Lock the mutex to access the shared data
            let mut data = data_clone.lock().unwrap();
            data.push(i); // Modify the data
            println!("Thread {}: {:?}", i, *data);
        });

        // Store the thread handle
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Final state of shared data
    let final_data = shared_data.lock().unwrap();
    println!("Final data: {:?}", *final_data);
}
