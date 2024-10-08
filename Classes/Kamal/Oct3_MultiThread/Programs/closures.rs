use std::thread;

fn conn_handler(comment:String) {
    println!("{}, Connection handler!!!", comment);
}

fn main() {
    let comment = String::from("Welcome!!!");
    let c1 = move|| {
        println!("Data passed is {}", comment);
        conn_handler(comment);
    };
    //println!("Exit!!! {}", comment);
}

