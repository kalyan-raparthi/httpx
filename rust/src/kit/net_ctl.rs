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

pub fn set_port(port: u16) {
    let f_str = fs::read_to_string(CONFIG_PATH).expect("ERROR READING CONFIG FILE");
    let mut new_str = String::new();
    for line in f_str.lines() {
        if line.starts_with("&PORT") {
            new_str.push_str(&format!("&PORT {}\n", port));
        } else {
            new_str.push_str(&format!("{}\n", line));
        }
    }
    match fs::write(CONFIG_PATH, new_str) {
        Ok(_) => println!("PORT NUMBER SET TO {}", port),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                println!("ERROR: CONFIG FILE NOT FOUND");
                std::process::exit(1);
            }
            std::io::ErrorKind::PermissionDenied => {
                println!("ERROR: PERMISSION DENIED / NEEDS ADMINISTRATOR PRIVILEGES");
                std::process::exit(1);
            },
            std::io::ErrorKind::AddrInUse => {
                println!("ERROR: PORT NUMBER ALREADY IN USE");
                std::process::exit(1);
            },
            _ => {
                println!("ERROR: ERROR SETTING PORT NUMBER");
                std::process::exit(1);
            }
            },
    }
}

pub fn set_ip(ip: &str) {
    let f_str = fs::read_to_string(CONFIG_PATH).expect("ERROR READING CONFIG FILE");
    let mut new_str = String::new();
    for line in f_str.lines() {
        if line.starts_with("&IP") {
            new_str.push_str(&format!("&IP {}\n", ip));
        } else {
            new_str.push_str(&format!("{}\n", line));
        }
    }
    match fs::write(CONFIG_PATH, new_str) {
        Ok(_) => println!("IP ADDRESS SET TO {}", ip),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                println!("ERROR: CONFIG FILE NOT FOUND");
                std::process::exit(1);
            }
            std::io::ErrorKind::PermissionDenied => {
                println!("ERROR: PERMISSION DENIED / NEEDS ADMINISTRATOR PRIVILEGES");
                std::process::exit(1);
            },
            _ => {
                println!("ERROR: ERROR SETTING IP ADDRESS");
                std::process::exit(1);
            }
            },
    }
}