use std::collections::HashMap;

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

fn get_file_size(path: &str) -> std::io::Result<u64> {
    let metadata = std::fs::metadata(path).expect("get_file_size: ERROR WHILE GETTING FILE METADATA");
    Ok(metadata.len())
}

fn response(mut stream: TcpStream) {
    // CLIENT BUFFERED READER
    let mut reader = std::io::BufReader::new(&mut stream);
    
    let mut request = String::new();
    let _ = reader.read_line(&mut request);
    

    let mut headers_str = String::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() || line == "\r\n" {
            break;
        }
        headers_str.push_str(&line);
    }
    let _headers = parse_headers_to_hashmap(&headers_str);

    match request.split_whitespace().next().unwrap() {
        "GET" => {
            if request.split_whitespace().nth(1).unwrap() == "/" {
                println!("METHOD: GET, PATH: /");
                get_html(&mut stream, "./index.html");
            }
            else {
                send_404(&mut stream);
            }
        }
        _ => { eprintln!("INVALID HTTP METHOD"); }
    }
}

fn get_html(stream: &mut TcpStream, path: &str) {

    let file = File::open(path).expect("ERROR WHILE OPENING FILE\n");
    
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n",
        get_file_size(path).expect("get_html: ERROR WHILE GETTING FILE SIZE")
    );
    stream.write_all(header.as_bytes()).expect("get_html: ERROER WHILE WRITING HEADERS TO CLINET\n");
    
    let mut buffered_writer = std::io::BufWriter::new(stream);
    std::io::copy(&mut BufReader::new(file), &mut buffered_writer).expect("get_html: ERROR WHILE WRITING CONTENT TO CLIENT\n");
    buffered_writer.flush().expect("get_html: ERROR WHILE FLUSHING CLI BUF_WRITER\n");
}

fn send_404(stream: &mut impl Write) {
    let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\n404 Not Found";
    stream.write_all(response.as_bytes()).expect("send_404_response: ERROR WHILE WRITING 404 RESPONSE");
}

pub fn parse_headers_to_hashmap(headers: &String) -> HashMap<String, String> {
    let mut header_map = HashMap::new();
    for line in headers.lines() {
        if let Some((key, value)) = line.split_once(": ") {
            header_map.insert(key.to_string(), value.to_string());
        }
        else if let Some((key, value)) = line.split_once("=") {
            header_map.insert(key.to_string(), value.to_string());
        }
    }
    header_map
}



fn main() {
    let server_socket = TcpListener::bind("127.0.0.1:2025").expect("Failed to bind");
    println!("SERVER LISTENING ON: http://{}", &server_socket.local_addr().unwrap());

    loop {
        match server_socket.accept() {
            Ok((stream, cli_addr)) => {
                println!("CONNECTED TO {}", cli_addr);
                response(stream);
                println!("[CONNECTION CLOSED]");
            },
            Err(e) => {
                eprintln!("ERROR ACCEPTING CONNECTIONS => {e}");
            }
        }
    }
}