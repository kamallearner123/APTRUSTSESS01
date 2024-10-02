// use auth_service;
// fn main() {
//     println!("Hello, world!");
//     let result = auth_service::verify_authentication("Kamal", "@123");
//     println!("Result = {:?}", result);
// }


use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let file = File::open("important.txt");
    let mut data:[u8] = [10000;0];

    let file = match file {
        Ok(f) => f.read(&data),
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            println!("❌ File not found! Creating a new one...");
            File::create("important.txt").unwrap_or_else(|error| {
                panic!("❌ Problem creating the file: {:?}", error);
            })
        },
        Err(error) => {
            panic!("❌ Error opening the file: {:?}", error);
        },
    };
}