
fn add_tail(data:&mut String) {
    data.push_str("...modified by function");
}

fn main() {
    let mut data = String::from("hello");
    let ref_data:&mut String = &mut data;
    ref_data.push_str("...added by ref_data");
    println!("ref_data = {}", ref_data);

    add_tail(&mut data); // data becomes active
    data.push_str("world");
    ref_data.push_str("..."); // ERROR: ref_data is inaccessible.
    println!("data = {}", data);
}