#[derive(Debug, Clone)]
struct Student {
    id: i32,
    std: i8,
    name: String 
}

fn parse (data: &String) {
    let token = String::new();
    for &letter in data.iter() {
        println!("{}", letter);
    }
}
fn main()
{
    let s1 = Student{id:1, std:2, name:String::from("Kamal:sri:tiji")};
    parse(&s1.name);
}
