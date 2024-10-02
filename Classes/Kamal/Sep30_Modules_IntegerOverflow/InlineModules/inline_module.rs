// Defining module
mod math {
    pub fn add(a:i32, b:i32) -> i32 {
        return a+b;
    }
    // Defining sub-module
    pub mod trignometric {
        pub fn trig_fun1() -> Option<String> {
            return Some(String::from("returning from trig_fun1"));
        }
    }
}

fn main() {
    // Calling function from module
    println!("Result = {}", math::add(1,2));

    //Calling function from sub module of "math"
    println!("trig_fun1 output = {:?}", math::trignometric::trig_fun1());
}