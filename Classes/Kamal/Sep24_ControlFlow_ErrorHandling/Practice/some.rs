

#[derive(Debug)]
enum MyOption <T> {
    Group(T), //Group
    None
}

fn get_result(name: &str) -> MyOption<String> {
    if name == "kamal" {
        return MyOption::Group("first".to_string()); //Returning member in enum
    } else if name == "sri" {
        return MyOption::Group("second".to_string());
    } else {
        return MyOption::None;
    }
}

fn get_result_i32(name: &str) -> MyOption<i32> {
    if name == "kamal" {
        return MyOption::Group(1); //Returning member in enum
    } else if name == "sri" {
        return MyOption::Group(2);
    } else {
        return MyOption::None;
    }
}

fn main() {
    match get_result("kamal") {
        MyOption::Group(val) => println!("val = {}", val),
        MyOption::None => println!("None")
    }
}

