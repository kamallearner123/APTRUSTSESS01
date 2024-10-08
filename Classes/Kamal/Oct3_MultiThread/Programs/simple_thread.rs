use std::thread;

fn main() {
    let t1: thread::JoinHandle<Result<i32, &str>> = thread::spawn(||{
        println!("Thread created !!!");
        let result: Result<i32, &str> = Ok(10);
        return result;
    });
    
    println!("Thread joined : {:?}", t1);

    match t1.join() {
        Ok(val) => println!("Return value from threa = {:?}", val),
        Err(msg) => println!("Returned error : {:?}", msg)
    }
}