use rand::Rng;
use opencv::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(1..=100);
    println!("Random number: {}", random_number);
}

// use serde::{Deserialize, Serialize};
// use serde_json;

// #[derive(Serialize, Deserialize, Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

// fn main() {
//     let person = Person {
//         name: String::from("Alice"),
//         age: 30,
//     };

//     // Serialize `person` to a JSON string
//     let json_string = serde_json::to_string(&person).unwrap();
//     println!("Serialized JSON: {}", json_string);

//     // Deserialize JSON string back to `Person`
//     let deserialized_person: Person = serde_json::from_str(&json_string).unwrap();
//     println!("Deserialized Person: {:?}", deserialized_person);
// }
