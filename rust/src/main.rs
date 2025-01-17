use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    let peer_addr = stream
        .peer_addr()
        .map(|addr| addr.to_string())
        .unwrap_or_else(|_| "Unknown".to_string());
    println!("Incoming connection from: {}", peer_addr);

    let mut reader = BufReader::new(stream);
    let mut request = String::new();

    pri
    println!("End of request.\n");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
