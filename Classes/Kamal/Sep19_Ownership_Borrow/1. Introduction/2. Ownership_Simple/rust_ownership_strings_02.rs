fn fun(val:&mut String) {
    println!("{}", val);
    val.push_str(" world");
}

fn main() {
    let mut s = String::from("Hello");
    let mut t = s; // moved



    // let mut u = t; // moved
    // fun(&mut u);
    println!("{}", u);
}