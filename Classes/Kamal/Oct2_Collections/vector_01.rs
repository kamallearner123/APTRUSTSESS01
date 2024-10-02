
fn to_int(s:&str) {
    s.parse().unwrap()
}

fn main() {
    let nums = vec!["1", "2"];
    let mut sum = 0;
    for s in nums {
        sum = sum + to_int(&s);
    }
    sum
}