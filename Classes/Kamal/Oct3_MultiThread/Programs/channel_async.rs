use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello from the thread!");
        // Send a value through the channel
        tx.send(val).unwrap();
    });

    // Receive the value from the channel
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
