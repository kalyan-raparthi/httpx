use std::fs;
use crate::kit::core::CONFIG_PATH;

pub fn get_ip() -> String {
    let f_str = fs::read_to_string(CONFIG_PATH).expect("ERROR READING CONFIG FILE");
    for line in f_str.lines() {
        if line.starts_with("&IP") {
            if let Some(ip) = line.split_whitespace().nth(1) {
                return ip.to_string();
            }
        }
    }
    String::from("127.0.0.1")
}

pub fn get_port() -> String {
    let f_str = fs::read_to_string(CONFIG_PATH).expect("ERROR READING CONFIG FILE");
    for line in f_str.lines() {
        if line.starts_with("&PORT") {
            if let Some(port) = line.split_whitespace().nth(1) {
                return port.to_string();
            }
        }
    }
    String::from("2025")
}