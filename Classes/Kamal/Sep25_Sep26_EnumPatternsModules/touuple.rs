fn example() -> (i32, f32, String) {
    (11, 10.1, String::from("Hello"))
}

fn main() {
    // let point = (0, 7, "string");
    // println!("{}", point.0);
    // println!("{}", point.0);
    let result = example();
    match result {
        (10,x,y) => println!("{},{}", x,y),
        _ => println!("Default"),
    }

}