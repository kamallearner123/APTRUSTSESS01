use std::thread;
use std::env;


fn thread_handler(arg: String) -> Result<i32, &'static str> {

    println!("Thread created !!!");
    println!("Argumen= {:?} 
        getting handled in thread id {:?}", arg, thread::current().id());
    panic!("Invalid file oprtation");
}
fn main() {

    let mut tids = vec![];
    for arg in env::args().skip(1) {
        let tid: thread::JoinHandle<Result<i32, &str>> = thread::spawn(move||{
            thread_handler(arg)
        });
        tids.push(tid);
    }

    for tid in tids {
        match tid.join() {
            Ok(_) => println!("Executed properly"),
            Err(msg) => println!("Error value from thread {:?}", msg)
        }
    }
}
