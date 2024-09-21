fn feed_see(seed : &mut i32) {
    let result = *seed+10;
    println!("Result = {}", result);
    *seed += 1;
}

fn main() {
    let mut a = 10;
    let b = &mut a;
    *b = *b+100;
    let c = 1000+(*b);

    println!("{b}, {c}");

    feed_see(&mut a);
    println!("a after calling function = {}", a);
}