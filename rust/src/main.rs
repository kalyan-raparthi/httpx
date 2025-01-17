use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

fn respose(mut stream: TcpStream) {
    // CLIENT BUFFERED READER
    
    let mut reader = std::io::BufReader::new(&mut stream);
    
    // let mut request = String::new();
    // let _ = reader.read_line(&mut request);
    
    let mut request_vec: Vec<u8> = Vec::new();
    let _ = reader.read_to_end(&mut request_vec);
    stream.flush().expect("ERROR WHILE FLUSHING STREAM");

    let request = String::from_utf8(request_vec).unwrap();
    println!("METHOD: {}\nPATH: {}", request.split_whitespace().next().unwrap(), request.split_whitespace().nth(1).unwrap());

    match request.split_whitespace().next().unwrap() {
        "GET" => {
            if request.split_whitespace().nth(1).unwrap() == "/" {
                println!("METHOD: {}\nPATH: {}", request.split_whitespace().next().unwrap(), request.split_whitespace().nth(1).unwrap());
                get_html(&mut stream, "./index.html");
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

fn get_html(stream: &mut impl Write, path: &str) {
    
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n",
        get_file_size(path).unwrap()
    );
    stream.write_all(header.as_bytes()).expect("get_html: ERROER WHILE WRITING HEADERS TO CLINET\n");
    
    let mut file = File::open(path).expect("ERROR WHILE OPENING FILE\n");
    // OPENING TARGET FILE AND WRITING TO CLIENT
    // std::io::copy(&mut file, stream).expect("get_html: ERROR WHILE WRITING CONTENT TO CLIENT\n");
    
    let mut buffered_writer = std::io::BufWriter::new(stream);
    std::io::copy(&mut BufReader::new(file), &mut buffered_writer).expect("get_html: ERROR WHILE WRITING CONTENT TO CLIENT\n");
    // buffered_writer.flush().expect("get_html: ERROR WHILE FLUSHING BUFFER\n");

}

fn get_download(stream: &mut impl Write, path: &str) {
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Disposition: attachment;\r\nContentContent-Length: {}\r\n\r\n",
        get_file_size(path).unwrap()
    );

    stream.write_all(header.as_bytes()).expect("get_donwload: ERROR WHILE WRITING TO CLIENT");
    let mut file = File::open(path).expect("ERROR WHILE OPENING FILE");
    std::io::copy(&mut file, stream).expect("get_download: ERROR WHILE WRITING TO CLIENT");
}

fn get_file_size(path: &str) -> std::io::Result<u64> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len())
}

fn send_400(stream: &mut TcpStream) {
    let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 11\r\nContent-Type: text/plain\r\n\r\nBad Request";
    if stream.write_all(response.as_bytes()).is_err() {
        eprintln!("Failed to write 400 response");
    }
}

fn send_500(stream: &mut TcpStream) {
    let response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 21\r\nContent-Type: text/plain\r\n\r\nInternal Server Error";
    if stream.write_all(response.as_bytes()).is_err() {
        eprintln!("Failed to write 500 response");
    }
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