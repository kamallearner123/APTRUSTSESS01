use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let data = Rc::new(RefCell::new(String::from("Hello")));

    let data1 = Rc::clone(&data);
    let data2 = Rc::clone(&data);

    data1.borrow_mut().push_str(", Rust!");

    println!("data1: {}", data1.borrow());
    println!("data2: {}", data2.borrow());
}
