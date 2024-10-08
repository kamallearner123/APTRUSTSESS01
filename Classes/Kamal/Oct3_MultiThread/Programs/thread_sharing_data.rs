use std::sync::Arc;
use std::thread;

fn main() {
    // Create some read-only data (a vector of integers)
    let mutdata = [1, 2, 3, 4, 5];

    // Wrap the data in Arc to allow shared ownership among threads
    let shared_data = Arc::new(data);

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    for i in 0..5 {
        // Clone the Arc to share the data with each thread
        let mut data_clone = Arc::clone(&shared_data);
        
        // Spawn a new thread
        let handle = thread::spawn(move || {
            // Each thread can read the shared data
            data_clone[0] = 100;
            println!("Thread {}: {:?}", i, data_clone);
        });

        // Store the thread handle
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
