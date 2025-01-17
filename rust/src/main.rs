use std::collections::HashMap;

use std::fs::File;
use std::io::{BufRead, BufReader, Write, BufWriter};
use std::net::{TcpListener, TcpStream};

fn get_file_size(path: &str) -> std::io::Result<u64> {
    let metadata = std::fs::metadata(path).expect("get_file_size: ERROR WHILE GETTING FILE METADATA");
    Ok(metadata.len())
}

fn response(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    
    let mut request = String::new();
    let _ = reader.read_line(&mut request);
    

    let mut headers_str = String::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() || line.trim().is_empty() {
            break;
        }
        headers_str.push_str(&line);
    }
    let _headers = parse_headers_to_hashmap(&headers_str);

    match request.split_whitespace().next().unwrap() {
        "GET" => { handle_get( &mut writer, request.split_whitespace().nth(1).unwrap());}
        _ => { eprintln!("INVALID HTTP METHOD"); }
    }
}

fn handle_get(writer: &mut BufWriter<&TcpStream>, path: &str) {
    let file_path = if path == "/" { "./index.html" } else { &path[1..] };

    if file_exists(file_path) {
        send_file(writer, file_path);
    } else {
        send_response(writer, 404, "Not Found", Some("404 Not Found"));
    }
}

fn send_file(writer: &mut BufWriter<&TcpStream>, path: &str) {
    let file = File::open(path).expect("ERROR WHILE OPENING FILE");
    
    let paths = path.split('/').last().unwrap();
    let response_type = paths.split('.').last().unwrap();

    writer.write_all(get_header(response_type, path).as_bytes()).expect("send_file: ERROR WHILE WRITING HEADERS TO CLIENT");
    std::io::copy(&mut BufReader::new(file), writer).expect("send_file: ERROR WHILE WRITING CONTENT TO CLIENT");
    writer.flush().expect("send_file: ERROR WHILE FLUSHING BUFFER");
}

fn main() {
    let server_socket: TcpListener = TcpListener::bind("127.0.0.1:2025").expect("Failed to bind");
    println!("SERVER LISTENING ON: http://{}", &server_socket.local_addr().unwrap());
    
    for stream in server_socket.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    let _cli_addr = stream.peer_addr().expect("Failed to get client address");
                    println!("CLIENT CONNECTED: {}", _cli_addr);``
                    response(stream);
                });
            },
            Err(e) => {
                eprintln!("ERROR ACCEPTING CONNECTIONS => {e}");
            }
        }
    }
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

fn file_exists(path: &str) -> bool {
    match std::fs::metadata(path) {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}

fn send_response(writer: &mut BufWriter<&TcpStream>, status_code: u16, status_message: &str, body: Option<&str>) {
    let body = body.unwrap_or("");
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        status_message,
        body.len(),
        body
    );
    writer.write_all(response.as_bytes()).expect("send_response: ERROR WHILE WRITING RESPONSE");
    writer.flush().expect("send_response: ERROR WHILE FLUSHING BUFFER");
}

fn get_header(response_type: &str, path: &str) -> String {
    let content_type = match response_type {
        "html" => "text/html",
        "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "png" => "image/png",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "ico" => "image/x-icon",
        "svg" => "image/svg+xml",
        "tiff" => "image/tiff",
        "webp" => "image/webp",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "mp4" => "video/mp4",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "rar" => "application/vnd.rar",
        "7z" => "application/x-7z-compressed",
        "txt" => "text/plain",
        "md" => "text/markdown",
        _ => "application/octet-stream",
    };
    
    let content_length = get_file_size(path).expect("get_header: ERROR WHILE GETTING FILE SIZE");
    
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        content_type, content_length
    )
}