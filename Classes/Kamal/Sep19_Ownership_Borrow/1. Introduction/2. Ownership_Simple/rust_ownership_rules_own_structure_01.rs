struct MyStruct {
    name: String,
    id: i32
}

impl MyStruct {
    // Constructor for the struct
    fn new(name: &str) -> MyStruct {
        MyStruct {
            name: name.to_string(),
            id: 10
        }
    }
}

// Implementing the Drop trait for MyStruct
impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct with name: {}", self.name);
    }
}

fn main() {
    // Creating an instance of MyStruct
    let my_struct_1 = MyStruct::new("Struct 1"); //r+w
    let my_struct_1 = MyStruct::new("Struct 2"); //r+w
    println!("Drop for Struct 1 should be called by now");
    


    // //let name_person_1 = my_struct_1.name;
    // //let name_person = my_struct_1.name.clone(); //
    // let id = my_struct_1.id; // Copy
    // my_struct_1.name.push_str(" .....");

    // // Moving ownership of my_struct_1 to my_struct_2
    // let my_struct_2 = my_struct_1; // r+w

    // // my_struct_1 is no longer valid here, but my_struct_2 owns the value.
    // println!("Ownership moved to my_struct_2 with name: {}", my_struct_2.name);

    // // my_struct_2 will be dropped when it goes out of scope (end of main).
}
