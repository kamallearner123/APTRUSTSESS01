fn method(n: i32) -> i32 {
    (0..=n).fold(0, |sum, i| sum+i)
}

fn main() {
    println!("{}", method(10));
}