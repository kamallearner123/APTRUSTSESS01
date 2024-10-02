use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create an Arc to share the counter across threads
    let counter = Arc::new(Mutex::new(0));
    let mut gCounter = Arc::new(0);

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    // Spawn 10 threads
    for _ in 0..10 {
        // Clone the Arc to share ownership of the counter
        let counter = Arc::clone(&counter);
        let gCounter = Arc::clone(&gCounter);

        let handle = thread::spawn(move || {
            // Lock the Mutex to get mutable access to the counter
            {   
                //gCounter = gCounter+1;
                println!("{}", gCounter);//only read
                //let localCounter = gCounter.unwrap();

                let mut num = counter.lock().unwrap(); //lock
                *num += 1;
            } //unlock
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    println!("Result: {}", *counter.lock().unwrap());
}
