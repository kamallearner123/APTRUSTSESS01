#[derive(Debug)]
enum COLORS {
    RED,
    GREEN,
    BLUE
}

#[derive(Debug)]
enum MyOption {
    Some(COLORS),
    None
}

fn GetColorCode(name: &str) -> MyOption {
    let mut result: MyOption = MyOption::None;
    if name == "RED" {
        result = MyOption::Some(COLORS::RED);
    } else if name == "BLUE" {
        result = MyOption::Some(COLORS::BLUE);
    }
    return result;
}

fn main() {
    match GetColorCode("BLUE") {
        MyOption::Some(Value) => println!("Hurray... Value = {:?}", Value),
        MyOption::None => println!("Color name is not existing!!!!")
    }
}