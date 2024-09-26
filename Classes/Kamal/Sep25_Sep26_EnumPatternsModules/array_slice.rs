fn match_array(array:&[i32]) {
    match array {
        &[1, .., 3] => println!("Array matches exactly [1, 2, 3]!"),
        &[1,2,..] => println!("Array starts with 1!"),
        &[.., 3] => println!("Array ends with 3!"),
        _ => println!("No match!"),
    }
}

fn main() {
    let array1:[i32;4] = [1,2,3,4];
    let array2:[i32;10] = [1,2,3,4,5,6,7,8,9,0];
    match_array(&array1);
    match_array(&array2);
}


// fn main() {
//     let array:[i32;3] = [1,24,3];
//     match array {
//         [1, 2, 3] => println!("Array matches exactly [1, 2, 3]!"),
//         [2, ..] => println!("Array starts with 1!"),
//         [.., 3] => println!("Array ends with 3!"),
//         _ => println!("No match!"),
//     }
//     println!("{:?}",array);
// }

