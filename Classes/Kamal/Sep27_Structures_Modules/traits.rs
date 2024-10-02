#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };
    let mut point2 = point.clone();
    let mut point3 = point;

    point2.x = 11;
    point3.x = 12;
    println!("{:?}", point);
    println!("{:?}", point2);
    println!("{:?}", point3);

    println!("Eq = {}", point == point2);
}
