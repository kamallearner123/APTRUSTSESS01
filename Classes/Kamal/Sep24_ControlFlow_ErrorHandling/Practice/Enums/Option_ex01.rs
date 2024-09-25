
// enum MyOption<T1,T2> {
//     Coorect(T1),
//     Alternative(T2),
//     None
// }

// fn find_name(search_tag: &str) -> MyOption<String,String> {

// }


#[derive(Debug)]
enum Color {
    RED,
    GREEN,
    YELLOW,
    ERROR(String) //24 bytes
}


fn find_color() -> Color {
    return Color::RED;
}
fn main() {
    let bus_color = find_color();
    println!("{:?}", bus_color);
}