fn main() {
    // Example with a string slice
    let my_string = String::from("Hello, Rust!");
    let slice = &my_string[0..5]; // Create a slice from the string
    print_slice(slice); // Pass the slice reference to the function

    // Example with an array slice
    let my_array = [1, 2, 3, 4, 5];
    let array_slice = &my_array[1..4]; // Create a slice from the array
    print_array_slice(array_slice); // Pass the array slice reference to the function
}

// Function that takes a string slice
fn print_slice(s: &str) {
    println!("String slice: {}", s);
}

// Function that takes an array slice
fn print_array_slice(s: &[i32]) {
    println!("Array slice: {:?}", s);
}
