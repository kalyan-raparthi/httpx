/*
    HTTPX - MAIN FILE V1.0
    AUTHOR: KALYAN RAPARTHI/ [ qb ], kalyan.raparthi@hotmail.com, Gihuub: @kalyan-raparthi
*/

// mod for core http functions
mod kit; 

use kit::core::{app_start, set_up};
use kit::net_ctl::{get_ip, get_port};
use std::env::args;

// HELP MESSAGE
const HELP: &str = 
r#"USAGE:

httpx <command> / <option>

start           starts httpx server
setup               sets up httpx server
view-ip             views the ip address of the server
view-port           views the port of the server
help                prints this help message
-v, --version       prints the version of httpx

"#;
// stop        stops httpx server

pub fn parse_args() -> () {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("{}", HELP);
        std::process::exit(1);
    }
    match args[1].as_str() {
        "setup" | "SETUP" => {
            set_up().expect("ERROR WHILE SETTING UP");
        }

        "start" | "START" => {
            app_start().expect("ERROR WHILE STARTING SERVER"); 
        }
        
        "stop" | "STOP"=> {
            println!("HTTPX TERMINATED");
        }

        "view-ip" | "VIEW-IP" => {
            println!("{}", get_ip());
        }

        "view-port" | "VIEW-PORT" => {
            println!("{}", get_port());
        }

        "help" | "HELP" => {
            eprintln!("{}", HELP);
        }
        
        "-v" | "--version" => {
            eprintln!("{}", std::env::var("CARGO_PKG_VERSION").unwrap());
        }
        _ => {
            eprintln!("Invalid command");
            std::process::exit(1);
        }
    }    
}

fn main() {
    parse_args();
}