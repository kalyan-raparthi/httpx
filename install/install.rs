use std::fs;
use std::path::Path;
use std::io::BufRead;

const DEFAULT_CONFIG: &str = "&VERSION 0.1.0\n&IP localhost\n&PORT 80\n";

fn main() {
    if fs::metadata("C:/Program Files/httpx").is_ok() {
        println!("HTTPX is already installed.");
        let reader = std::io::BufReader::new(fs::File::open("C:/Program Files/httpx/config.txt").expect("Failed to open config.txt"));
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("&VERSION") {
                    println!("HTTPX version: {}", line.split_whitespace().last().unwrap());
                }
            }
        }
        return;
    }

    let home: &str = "C:/Program Files/";
    let httpx_path = Path::new(home).join("httpx");

    fs::create_dir_all(&httpx_path).expect("FAILED TO CREATE HTTPX DIRECTORY");
    let config_path = httpx_path.join("config.txt");
    
    fs::write(config_path, DEFAULT_CONFIG).expect("FAILED TO WRITE CONFIG FILE");
    fs::write(httpx_path.join("index.html"),
        r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Welcome to HTTPX</title>
            <style>
                body {
                    font-family: monospace;
                    background-color: #f4f4f4;
                    margin: 0;
                    padding: 0;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                }
                .container {
                    text-align: center;
                    background: #fff;
                    padding: 20px;
                    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
                    border-radius: 8px;
                }
                h1 {
                    color: #333;
                }
                p {
                    color: #666;
                }
                a {
                    display: inline-block;
                    margin-top: 20px;
                    padding: 10px 20px;
                    color: #fff;
                    background-color: #007BFF;
                    text-decoration: none;
                    border-radius: 5px;
                }
                a:hover {
                    background-color: #0056b3;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Welcome to HTTPX</h1>
                <p>Your HTTP server is up and running!</p>
            </div>
        </body>
        </html>
        "#).expect("Failed to write index.html");
}