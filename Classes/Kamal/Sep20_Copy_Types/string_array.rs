fn main() {
    let s = String::from("It is Rust");
    let t = s;
    //println!(" s = {:?}", s); //ERROR

    let array1 = [1,2,3,4];
    let array2 = array1; //No issue: No ownership is moved
    println!("array1 = {:?}", array1);
}