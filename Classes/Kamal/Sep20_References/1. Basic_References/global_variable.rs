
static mut counter:i32 = 10;

fn main() {
    unsafe {
        counter = counter+1;
        println!("Counter = {}", counter);
    }
}