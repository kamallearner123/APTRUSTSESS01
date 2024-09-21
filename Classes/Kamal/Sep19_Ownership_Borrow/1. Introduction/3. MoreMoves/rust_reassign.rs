fn main() {
    let mut s = String::from("Hello");
    let t = s; // Moved to "t"
    s = String::from("World"); // 's' assigned with new value
    println!("{s}, {t}"); // Nothing is dropped.
}