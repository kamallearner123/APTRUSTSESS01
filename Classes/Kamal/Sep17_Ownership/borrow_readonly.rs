use std::mem::drop;

fn main() {
    /*
    Ownership overview
    Move semantics
    Borrowing basics
    Borrow checker
    Mutable Vs Immutable borrowing
    Slices and borrowing
    */
    
    let mut a = String::from("Hello");
    let b = &a;
    let c = &a;

    println!("{a}, {b}, {c}"); // No issue: 

    //a.push_str(" world");

    let b = &mut a;
    b.push_str("..");
    drop(b);
    drop(c);

    println!("{a}"); // ERROR
}