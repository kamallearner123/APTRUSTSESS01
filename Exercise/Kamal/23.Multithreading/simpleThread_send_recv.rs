use std::{thread};
use std::time::Duration;

fn main() {
    let count = 10;

    let t1 = std::thread::spawn(move||{
        let mut count1 = count;
        for i in 0..count1 {
            println!("Count = {}", count1);
            count1 = count1-1;
            thread::sleep(Duration::from_millis(1));
        }
    });

    t1.join().unwrap();
}