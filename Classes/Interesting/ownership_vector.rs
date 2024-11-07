fn main() {
    let mut v1 = vec![1,2,3];
    let num = &v1[0];
    println!("num = {}", num);

    v1.push(4);
    println!("v1 = {:?}", v1);

    println!("num = {}", num);
}


