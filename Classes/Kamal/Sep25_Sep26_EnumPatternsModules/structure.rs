enum MyOption <T> {
    Some(T),
    SOMEOTHER,
    None
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn func() -> MyOption<Point> {
    MyOption::Some(Point{x:0, y:2})
    //MyOption::None
}
fn main() {
    match func() {
        MyOption::Some(Point { x: 0, y }) => println!("On the y-axis at y = {}", y),
        MyOption::Some(Point { x, y: 0 }) => println!("On the x-axis at x = {}", x),
        // Some(Point { x, y }) => println!("Point is at ({}, {})", x, y),
        //MyOption::Some(x) => println!("{:?}", x),
        MyOption::None => println!("Nothing"),
        MyOption::SOMEOTHER => todo!(),
        _ => println!(""),
    }
}
