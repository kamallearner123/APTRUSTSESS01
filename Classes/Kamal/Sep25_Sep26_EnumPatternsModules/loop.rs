fn main() {
    let mut count = 10;
    
    let a = loop {
        println!("in loop");
        if count <= 0 {
            break 0;
        }

        count -= 1;
    };

    println!("a = {}", a);
}