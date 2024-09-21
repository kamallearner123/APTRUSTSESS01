
// fn main() {
//     let mut a = 10.1;
//     let b = 20;
//     let data = Box::new(100);
//     a = 1_i32.into();
    
//     // println!("a = {}", a);
//     // *a = 200;
//     // println!("a = {}", a);
// }


// fn main() {
//     // Original ownership
//     let original = String::from("Hello, Rust!");

//     // Ownership is moved to new_owner
//     let new_owner = original;

//     // Ownership can be passed back manually
//     let original = new_owner;

//     println!("{}", original);  // Now original owns the value again
// }



#[derive(Debug)]
struct Student {
    name: String, // No mut is added
    id:i32, // No mut is added 
}

impl Student {
    fn new(name :&str, sid: i32) -> Self {
        Student {
            name: name.to_string(),
            id: sid
        }
    }
}

fn main() {
    let mut s1 = Student::new("Kamal", 13);
    let mut class = Vec::new();
    s1.name.push_str(" kumar"); // Modified.
    println!("{:?}", s1);
    class.push(s1);

    //Mutable Borrow
    let mut element = &mut class[0];
    //Edit the content: No error
    element.name.push_str(" mukiri");
    println!("{:?}", element);
    println!("{:?}", class[0]);
}


