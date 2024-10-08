fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    // Unsafe block that creates a dangling reference
    let r: *const i32 = &vec[0];

    // This will cause a use-after-free issue
    drop(vec);

    unsafe {
        // Dereferencing a dangling pointer
        println!("Dangling reference: {}", *r);
    }
}

