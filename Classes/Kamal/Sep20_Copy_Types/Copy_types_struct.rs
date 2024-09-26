#[derive(Debug)]
struct IdsSensor {
    id: i32,
    severity: i8
}

fn main() {
    let device1 = IdsSensor{id:1234, severity:0};
    let device2 = device1;

    print
}