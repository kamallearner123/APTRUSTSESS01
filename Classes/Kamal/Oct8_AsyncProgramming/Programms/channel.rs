use std::sync::mpsc; // Import the multi-producer, single-consumer channel
use std::thread;
use std::time::Duration;

fn main() {
    // Create a new channel
    let (tx, rx) = mpsc::channel();

    // Create a producer thread
    let producer = thread::spawn(move || {
        for i in 1..6 {
            // Send data through the channel
            tx.send(i).unwrap();
            println!("Produced: {}", i);
            thread::sleep(Duration::from_secs(1)); // Simulate work
        }
    });

    // Create a consumer thread
    let consumer = thread::spawn(move || {
        for _ in 1..6 {
            // Receive data from the channel
            let received = rx.recv().unwrap();
            println!("Consumed: {}", received);
        }
    });

    // Wait for both threads to finish
    producer.join().unwrap();
    consumer.join().unwrap();
}
