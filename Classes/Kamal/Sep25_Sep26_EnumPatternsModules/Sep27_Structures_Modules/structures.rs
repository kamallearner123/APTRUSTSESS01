#[derive(Debug)]
struct IdsData (
    i32, f32, String);

fn main() {
    let event = IdsData(1, 1.1, String::from("ssh signing failure"));
    println!("{:?}", event.0);
}


