fn main() {
    let mut array = String::from("0123456789");
    let slice1 = &array[3..]; // [ 0, 1, 2, 3, 4, 5, 6 ]
    let slice2 = &array[0..9]; // [ 0, 1 ]

    println!("{slice1}, {slice2}");
}
