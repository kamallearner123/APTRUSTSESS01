trait Greet {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, my name is {}!", self.name)
    }
}

fn say_hello<T: Greet>(entity: &T) {
    println!("{}", entity.greet());
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    println!("{}", person.greet());

    say_hello(&person);
}
