use std::net::TcpStream;
use std::time::Duration;
use std::io::{Read, Write};
use std::thread;

fn client_fun() {
    let mut client = TcpStream::connect("192.168.0.112:7878").unwrap();
    println!("Connected to the server!");
    loop {
        let mut in_msg = String::new();
        client.read_to_string(&mut in_msg).unwrap();
        println!("Server: {}", in_msg);
        //client.send("Hello from client!".as_bytes()).unwrap();
        client.write("Hello from client!".as_bytes()).unwrap();
        Duration::from_secs(1);
    }
}


fn main() {
    let mut clients = Vec::new();
    for _ in 0..10 {
        let client = thread::spawn(|| {
            client_fun();
        });
        clients.push(client);
    }


    for client in clients {
        client.join().unwrap();
    }
}