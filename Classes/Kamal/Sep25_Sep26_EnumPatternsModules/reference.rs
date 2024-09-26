fn match_ref_fun(value1:i32) {
    let mut value = value1;
    match value {
        ref mut val => {
            *val += 1;
            println!("Modified value is now {}", val);
        }
    }
}

fn main() {
    let a =10;
    match_ref_fun(a);
}