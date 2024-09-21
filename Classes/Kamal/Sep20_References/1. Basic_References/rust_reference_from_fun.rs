fn feed_see() ->&i32 {
    let a= 10;
    return &a;
}

fn main() {
    let ref: &i32 = fun();
}