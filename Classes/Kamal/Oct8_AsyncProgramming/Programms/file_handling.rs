use std::fs::OpenOptions;

fn main() {
    let fd = OpenOptions::new().read(true)
            .write(true)
            .create(false)
            .open("foo.txt");

    // pub enum ErrorKind
    match fd {

        match fd {
            NotFound => println!("Not found"),
            _ => println!("Default!!!")
        }

}

