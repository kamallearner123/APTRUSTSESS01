use std::sync::Arc; //1
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn thread_handler(num: &i32, data: Arc<Mutex<String>>) -> Result<i32, String> {
    println!("Thread handler!!!! {}", num);
    let mut g_data = data.lock().unwrap(); //
                                           // data.lock(): reference to location or Error
    g_data.push_str(&num.to_string());
    thread::sleep(Duration::from_millis(5000));
    Ok(100)
}

fn main_fun() {
    println!("In main function handler!!!!");
}

/*
1) Concurrency
1,000,000 :
2) Performance :
*/

fn main_func() {
    let mut tids = vec![];
    let mut message = Arc::new(Mutex::new(String::from("It is Rust!!"))); //2
                                                                          // Creating thread
    for i in 0..10 {
        //let mut msg = message.clone(); // Creating new value and assigning to msg
        //msg.push_str(&i.to_string());
        let msg = Arc::clone(&message); // 3:

        tids.push(thread::spawn(move || {
            thread_handler(&i, msg) //return value from thread
        }));
    }
    main_fun();
    for tid in tids {
        match tid.join() {
            Ok(val) => println!("Result = {:?}", val),
            Err(msg) => println!("Error emssage = {:?}", msg),
        }
    }

    println!("Final message : {}", message.lock().unwrap());
    println!("Exit!!!")
}

fn main() {
    main_func();
}
