
fn main() {
    let (mut a, mut b) = (String::from("Team"), 
                        vec!["sachin", 
                            "dravid", 
                            "ganguly"]);
    a.push_str(" Cricket");


    // There is a chance that 'pop' may return None from Option<T>
    let Some(last) = b.pop() else { panic!();};
    println!("last = {:?}", last);

    let akhari = b.pop() else {panic!();};
    println!("Fifth = {:?}", akhari);
}
