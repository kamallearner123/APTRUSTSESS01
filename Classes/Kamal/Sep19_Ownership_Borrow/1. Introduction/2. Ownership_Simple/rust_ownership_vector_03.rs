fn main() {
    let s = String::from("It is Rust");
    let t = s; // moving to t
    let u = t; // moving to u

    println!("{:?}", u);
}