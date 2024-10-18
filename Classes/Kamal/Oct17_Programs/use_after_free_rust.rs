use std::io::{self, Write};

struct MyData {
    info: String,
}

impl Drop for MyData {
    fn drop(&mut self) {
        println!("Dropping MyData with info: {}", self.info);
    }
}

fn main() {
    // Dynamically allocate memory on the heap using Box
    let mut data = Box::new(MyData {
        info: String::from("This is dynamically allocated memory."),
    });
    unsafe{
        loop {
            // println!("Memory is allocated with data: {}", data.as_ref().unwrap().info);

            // Prompt the user whether to drop the memory or not
            println!("Do you want to drop the allocated memory? (y/n): ");
            io::stdout().flush().unwrap();  // Ensures prompt is shown before input

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "y" {
                // Manually drop the memory
                drop(data);  // Use take() to move out the value and drop it
                println!("Memory has been dropped.");
                let mut data2 = Some(Box::new(MyData {
                    info: String::from("This is dynamically allocated memory."),
                }));
                //break;  // Exit the loop after dropping the memory
            } else if input == "n" {
                println!("Memory is not dropped, you can still access it.");
                println!("Memory is allocated with data: {}", data.as_ref().info);

            } else {
                println!("Memory is allocated with data: {}", data.as_ref().info);
                println!("Invalid option. Please enter 'y' or 'n'.");
            }
        }
    }
    println!("Program finished.");
}
