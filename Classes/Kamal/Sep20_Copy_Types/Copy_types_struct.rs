fn main() {
    let array1 = [1,2,3,4];
    let array2 = array1; //  No ownership is moved

    println!("array1 = {:?}, array2 = {:?}", array1, array2);
}