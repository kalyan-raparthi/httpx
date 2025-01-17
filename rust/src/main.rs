use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    let mut buffer = [0; 1024];

    if let Ok(size) = reader.read(&mut buffer) {
        let request = std::str::from_utf8(&buffer[0..size]).unwrap_or("");
        if let Some(line) = request.lines().next() {
            if let Some(path) = line.split_whitespace().nth(1) {
                if path == "/" {
                    send_file(&mut stream, "index.html");
                } else {
                    send_file(&mut stream, path); 
                }
            }
        }
    } else {
        eprintln!("Failed to read from client");
    }
}

fn send_file(mut stream: &mut TcpStream, path: &str) {
    let mut writer = BufWriter::new(&mut stream);
    let file_path = PathBuf::from(path); 

    if file_path.is_file() {
        match File::open(&file_path) {
            Ok(file) => {
                let mut file_reader = BufReader::new(file);
                let mut buffer = [0; 1024];
                loop {
                    match file_reader.read(&mut buffer) {
                        Ok(0) => break, // End of file
                        Ok(bytes_read) => {
                            if let Err(e) = writer.write_all(&buffer[..bytes_read]) {
                                eprintln!("Error writing to client: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading from file: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                let response = b"HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n<h1>404 Not Found</h1>";
                if let Err(e) = writer.write_all(response) {
                    eprintln!("Error writing 404 response: {}", e);
                }
            }
        }
    } else {
        let response = b"HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n<h1>404 Not Found</h1>";
        if let Err(e) = writer.write_all(response) {
            eprintln!("Error writing 404 response: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Server listening on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}