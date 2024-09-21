
fn print(val: Vec<i32>) {
    println!("{}", val[0]);
}
fn main() {
    let s = vec![1,2,3];
    let flag = false;
    
    if flag == true {
        print(s);
    }
    // println!("{:?}", s); // ERROR: value borrowed here after move
}

