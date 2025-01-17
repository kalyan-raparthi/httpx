use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn respose(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    let _ = reader.read_line(&mut request_line);
    
    match request_line.split_whitespace().next().unwrap() {
        "GET" => {
            if request_line.split_whitespace().nth(1).unwrap() == "/" {
                send_html(&mut stream, "./index.html");
            }
            else {
                stream.write("404".as_bytes()).expect("ERROR WHILE WRITING");
            }
        }
        _ => {
            eprintln!("INVALID HTTP METHOD");
        }
    }
}

fn send_html(stream: &mut impl Write, path: &str) {
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n",
        get_file_size(path).unwrap()
    );

    stream.write_all(header.as_bytes()).expect("ERROER WHILE WRITING TO CLIENT");
    let mut file = File::open(path).expect("ERROR WHILE OPENING FILE");
    std::io::copy(&mut file, stream).expect("ERROR WHILE WRITING TO CLIENT");
}

fn send_download(stream: &mut impl Write, path: &str) {
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Disposition: attachment;\r\nContentContent-Length: {}\r\n\r\n",
        get_file_size(path).unwrap()
    );

    stream.write_all(header.as_bytes()).expect("ERROER WHILE WRITING TO CLIENT");
    let mut file = File::open(path).expect("ERROR WHILE OPENING FILE");
    std::io::copy(&mut file, stream).expect("ERROR WHILE WRITING TO CLIENT");
}

fn get_file_size(path: &str) -> std::io::Result<u64> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len())
}

fn main() {
    let server_socket = TcpListener::bind("127.0.0.1:2025").expect("Failed to bind");
    println!("SERVER LISTENING ON: http://{}", &server_socket.local_addr().unwrap());

    loop {
        match server_socket.accept() {
            Ok((stream, cli_addr)) => {
                println!("CONNECTED TO {}", cli_addr);
                respose(stream);
                println!("[CONNECTION CLOSED]");
            },
            Err(e) => {
                eprintln!("ERROR ACCEPTING CONNECTIONS => {e}");
            }
        }
    }
}