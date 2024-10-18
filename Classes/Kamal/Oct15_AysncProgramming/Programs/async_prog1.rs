
use async_std::task;
use std::time::Duration;

async fn download_data() {
    println!("Downloading data!!!");
    task::sleep(Duration::from_secs(10));
    println!("Downloading is done!!!!");
}


fn main() {
    println!("Start calling async functionsQQQ");
    let result = download_data();
    println!("Exting from the program!!!");
}