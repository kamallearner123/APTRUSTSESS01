use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct mpsc_1 {
    
}

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // int rxtx1[2] = pipe();
    // int rxtx2[2] = pipe();
    // //  child: rxtx1[0] - read
    // //  child: rxtx2[1] -  write
    // //  parent: rxtx1[0] - write
    // //  parent: rxtx2[1] - read

    thread::spawn(move || {
        let val = String::from("Hello from the thread!");
        // Send a value through the channel
        tx.send(val).unwrap();
    });

    // Receive the value from the channel
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
