
fn main() {
    let mut data = String::from("hello");
    let mut data_ref1 = &mut data;
    let data_ref2 = &data;
    data.push_str("... modified by owner");
    let data_ref3 = &data;
    let data_ref4 = &data;
    println!("{data}, {data_ref4}");


    let mut_ref = &mut data;
    mut_ref.push_str(".. added by mutable reference"); 
    println!("{data}");
}


// fn main() {
//     let mut data = String::from("hello");
//     println!("{data}");


//     let mut_ref = &mut data;
//     mut_ref.push_str(".. added by mutable reference"); 
//     //println!("{data}");
//     println!("{mut_ref}");
// }
