#[derive(Debug)]
enum COLORS {
    RED,
    GREEN,
    BLUE,
    NOCOLOR,
    MESSAGE(String)
}

fn GetColorCode(name: &str) -> COLORS {
    let mut result = COLORS::NOCOLOR;
    if name == "RED" {
        result = COLORS::RED;
    } else if name == "BLUE" {
        result = COLORS::BLUE;
    } else {
        result = COLORS::MESSAGE("Issue in finding color".to_string());
    }
    return result;
}

//errno, perror("")//

fn main() {
    match GetColorCode("BLACK") { // Switch
        COLORS::BLUE => println!("BLUE COLOR......"),
        COLORS::GREEN => println!("GREEN COLOR......"),
        COLORS::MESSAGE(msg) => println!("Message from fun - {}", msg),
        _ => println!("RED COLOR......")
    }
}

