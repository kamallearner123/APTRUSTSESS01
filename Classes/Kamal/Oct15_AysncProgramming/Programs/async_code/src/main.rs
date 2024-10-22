// use async_std::task;
// use async_std::Duration;

// async fn fun1() {
//     println!("Start func1");
//     std::time::sleep(Duration::from_secs(10));
//     println!("End func1");
// }

// fn main() {
//     let result = fun1();
//     println!("Called function!!");
// }


use async_std::task;
use std::time::Duration;

// Simulate a data download function
async fn download_data() -> String {
    println!("Starting to download data...");
    task::sleep(Duration::from_secs(2)).await;
    println!("Data downloaded!");
    String::from("Downloaded data")
}

// Simulate a data processing function
async fn process_data(data: String) {
    println!("Processing: {}", data);
    task::sleep(Duration::from_secs(1)).await;
    println!("Data processed!");
}

// Main function to coordinate the download and processing
async fn download_and_process() {
    let data = download_data().await;
    process_data(data);
}

fn main() {
    unsafe {
        let x: *const i32 = std::ptr::null();
        println!("{}", *x);  // Dereferencing a null pointer
    }
}

