use serde::{Deserialize, Serialize};
use serde_json::Result as JsonResult;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    amount: f64,
    from_account: String,
    to_account: String,
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Client disconnected
                println!("Client disconnected");
                break;
            }
            Ok(bytes_read) => {
                let message = String::from_utf8_lossy(&buffer[0..bytes_read]);
                println!("Received message: {}", message);

                // Parse the message as JSON
                match serde_json::from_str::<Transaction>(&message) {
                    Ok(transaction) => {
                        println!("Parsed transaction: {:?}", transaction);
                        let response = format!(
                            "Transaction from {} to {} for amount {} received.",
                            transaction.from_account, transaction.to_account, transaction.amount
                        );
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {}", e);
                        let error_msg = "Invalid JSON format received.";
                        stream.write_all(error_msg.as_bytes()).unwrap();
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected");
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept client: {}", e);
            }
        }
    }

    Ok(())
}
