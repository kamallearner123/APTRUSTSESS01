mod math;
fn main() {
    println!("Hello, world!");
    //calling funciton "add" from module "math"
    println!("Result = {}", math::basic::add(1,2));
}
