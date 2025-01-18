
struct node {
    next:Option<Box<node>>,
    data:i32
}

fn main() {
    let msg : &str = "Hello";
    let who: String = String::from("World");
    
    let refstr = &who;
    let final1 = String::from(refstr);
    
    println!("refstr = {refstr}, {final1}");
}

#####################

use serde_json; // 1.0.135
use serde_json::Value;


struct student {
    name:String,
    roll:i32
}
fn main () {

    let json_data = r#"
    {
    "Name":"Kamal"
    }"#;
    println!("json data = {json_data}");
    
    let parse: Value= serde_json::from_str(json_data).unwrap();

    println!("Value = {}", parse["Name"]);
    
    let values: student = student{name:parse["Name"].to_string(),
        roll:123};
    
}
####################
