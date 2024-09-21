fn main() {
    // let mut a= 10;
    // let mut b = &mut a; // a lost ownershil to 10
    // println!("{b}"); // No Issue
    // *b = 20;
    // println!("{b}"); // No Issue

    let mut Str1 = String::from("Hello ");
    let mut Str2 = &mut Str1;
    Str1.push_str(" world"); // ERROR: Str1 can not change the data
    //Str2.push_str(" 2");
    println!("{Str1}, {Str2}");
}