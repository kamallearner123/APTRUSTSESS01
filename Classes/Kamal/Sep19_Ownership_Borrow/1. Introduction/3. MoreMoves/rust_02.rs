#[derive(Debug)]
struct Person {
    name: String,
    birth: i32
}

fn main() {
    let mut composers: Vec<Person> = Vec::new(); // Allocating empty vector
    let p1 = Person{name:String::from("David"), birth:30};
    composers.push(p1);
    //println!("{:?}",p1); // ERROR: It is already moved.
    println!("{:?}", composers[0]);
}