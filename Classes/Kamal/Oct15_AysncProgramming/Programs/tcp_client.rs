use std::net::TcpStream;
use std::time::Duration;
use std::io::{Read, Write};
use std::thread;
use std::env;

fn client_fun() {
    let mut client = TcpStream::connect("192.168.0.117:7878").unwrap();
    println!("Connected to the server!");
    let mut buffer = [0;5];
    loop {
        
        client.read(&mut buffer).unwrap();
        println!("Server: {:?}", buffer);
        client.write("Hello from client!".as_bytes()).unwrap();
        println!("Sent to server!!!");
        thread::sleep(Duration::from_secs(1));
    }
}


fn main() {
    let mut clients = Vec::new();
    let args: Vec<String> = env::args().collect();
    for _ in 0..i32::from(args[0])() {
        let client = thread::spawn(|| {
            client_fun();
        });
        clients.push(client);
    }


    for client in clients {
        client.join().unwrap();
    }
}

