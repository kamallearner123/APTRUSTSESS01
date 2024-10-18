use std::io::prelude::*;
use std::net;

fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut socket = net::TcpStream::connect((host, port))?;
    println!("{:?}", socket);

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, "");
    socket.write_all(request.as_bytes());
    socket.shutdown(net::Shutdown::Write);

    let mut buffer = String::new();
    socket.read_to_string(&mut buffer);

    Ok(buffer)
}

fn main() {
    let result = cheapo_request("192.168.0.117", 7878, "/index.html");
    match result {
        Ok(data) => println!("Data received = {}", data),
        Err(msg) => println!("Error message = {}", msg)
    }
}
