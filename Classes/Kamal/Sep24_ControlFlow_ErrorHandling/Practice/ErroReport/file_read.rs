use std::fs::File;
use std::io::{self, Read};

fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;  // Attempts to open the file
    let mut content = String::new();
    file.read_to_string(&mut content)?;  // Attempts to read the file content
    Ok(content)
}

fn main() {
    let filename = "example.txt";

    match read_file_content(filename) {
        Ok(content) => println!("File content:\n{}", content),
        Err(error) => println!("Error reading the file: {}", error),
    }
}
