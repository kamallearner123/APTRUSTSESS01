use std::net::{TcpListener};
use std::thread;
use std::time::Duration;
use std::io::{Read, Write};

fn main() {
    let mut threads = Vec::new();
    let listener = TcpListener::bind("192.168.0.117:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok (mut stream_sock) => {
                let tid = thread::spawn (
                    move || {
                        println!("Connected to the client!");
                        let mut rcv_msg = [0;100];
                        loop {
                            stream_sock.write("Hello dear client".as_bytes()).unwrap();
                            println!("Written data to client!!!");
                            stream_sock.read(&mut rcv_msg).unwrap();
                            println!("Client: {:?}", rcv_msg);
                            Duration::from_secs(1);
                        }
                    }
                );
                threads.push(tid);
            },
            Err(err) => println!("Error: {err}")
        }
    }

    for thread in threads {
        thread.join().unwrap();
    }
}