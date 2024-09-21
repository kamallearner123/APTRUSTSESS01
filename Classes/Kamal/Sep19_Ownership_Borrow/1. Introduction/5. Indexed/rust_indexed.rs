fn main() {
    let names = Vec::new();
    
    for i in 0..100 {
        names.push(i.to_string());
    }
    
    let first = names[0];
    //println!("{}", names[0]); //ERROR: move occurs because 
    //value has type `String`, which does not implement the `Copy` trait
}
