use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client_connection(mut conn: TcpStream) {

    let mut msg: [u8;512] = [0;512];
    conn.read(&mut msg);
    println!("Received message: {:?}", msg);

}

fn main() {
    let listener = TcpListener::bind("192.168.0.117:7878").unwrap();
    let mut threads = Vec::new();

    for client_soc in listener.incoming() {
        println!("Socket details : {:?}", client_soc);

        match client_soc {
            Ok(conn) => {
                let tid = thread::spawn(||{
                    handle_client_connection(conn);
                });
                threads.push(tid);
            },
            Err(msg) => println!("Error while accepting the conn!!!")
        }


    }

    for tid in threads {
        tid.join();
    }
}