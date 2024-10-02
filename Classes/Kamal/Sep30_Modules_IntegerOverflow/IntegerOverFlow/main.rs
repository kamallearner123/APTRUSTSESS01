fn main() {
    // let x: u8 = 255;
    // let y = x.checked_add(10); //None incase of overflow
    // match y {
    //     Some(val) => println!("Result: {}", val),
    //     None => println!("Overflow occurred!"),
    // }

    let x: u8 = 255;
    let y = x.wrapping_add(10); // `y` will be `0`.
    println!("Result: {}", y);

    // let x: u8 = 255;
    // let y = x.saturating_add(100); // `y` will be `255`.
    // println!("Result: {}", y);
    
    // let x: u8 = 255;
    // let (y, overflow) = x.overflowing_add(10); // `y` will be `0` and `overflow` will be `true`.
    // println!("Result: {}, Overflow occurred: {}", y, overflow);

    // let mut count:u8 = 0;
    // loop {
    //     count = count + 1;
    //     if count>255 {
    //         break;
    //     }
    // }
}