const VERSION: &str = "0.1.0";
/*
    HTTPX - MAIN FILE V0.1.0
    AUTHOR: KALYAN RAPARTHI/ [ qb ], kalyan.raparthi@hotmail.com, Gihuub: @kalyan-raparthi
*/

// mod for core http functions
mod kit; 

use kit::core::app_start;
use kit::net_ctl::{get_ip, get_port};
use std::env::args;
use std::io::BufRead;

// HELP MESSAGE
const HELP: &str = 
r#"
    USAGE:

    httpx <command> / <option>

    start                   starts httpx server
        -p, --port          sets the port number
        -v, --verbose       prints the server details
    set-ip                  sets the ip address of the server
    set-port                sets the port number of the server

    config                  views the configuration of the server
    view-ip                 views the ip address of the server
    view-port               views the port of the server
    help                    prints this help message
    -v, --version           prints the version of httpx

"#;
// stop        stops httpx server

pub fn parse_args() -> () {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("{}", HELP);
        std::process::exit(1);
    }
    match args[1].as_str() {
        "start" | "START" => {
            if args.len() > 2 {
                match args[2].as_str() {
                    "-p" | "--port" => {
                        if args.len() > 3 {
                            match  args[3].parse::<u16>() {
                                Ok(port) => {
                                    kit::net_ctl::set_port(port);
                                }
                                Err(e) => match e.kind() {
                                    std::num::IntErrorKind::InvalidDigit => {
                                        println!("Error: Invalid PORT NUMBER");
                                        std::process::exit(1);
                                    }
                                    std::num::IntErrorKind::Empty => {
                                        println!("Error: THE INPUT IS EMPTY");
                                        std::process::exit(1);
                                    }
                                    _ => {
                                        println!("Error: INVALID PORT NUMBER");
                                        std::process::exit(1);
                                    }
                                },
                            }
                        } else if args.len() == 4 {
                            match args[4].as_str() {
                                "-v" | "--verbose" => {
                                    println!("HTTPX SERVER V{}\tLIVE ON http://{}:{}", VERSION, get_ip(), get_port());
                                }
                                _ => {
                                    eprintln!("INVALID OPTION");
                                    std::process::exit(1);
                                }
                            }
                        }
                    }

                    "-v" | "--verbose" => {
                        println!("HTTPX SERVER V{}\tLIVE ON https://{}:{}", VERSION, get_ip(), get_port());
                    }
                    _ => {
                        eprintln!("INVALID OPTION");
                        std::process::exit(1);
                    }
                }
            }
            app_start().expect("ERROR WHILE STARTING SERVER"); 
        }

        "set-ip" | "SET-IP" => {
            if args.len() > 2 {
                let ip = args[2].as_str();
                kit::net_ctl::set_ip(ip);
            } else {
                eprintln!("ERROR: INVALID IP ADDRESS");
                std::process::exit(1);
            }
        }

        "set-port" | "SET-PORT" => {
            if args.len() > 2 {
                match args[2].parse::<u16>() {
                    Ok(port) => {
                        kit::net_ctl::set_port(port);
                    }
                    Err(e) => match e.kind() {
                        std::num::IntErrorKind::InvalidDigit => {
                            println!("Error: Invalid PORT NUMBER");
                            std::process::exit(1);
                        }
                        std::num::IntErrorKind::Empty => {
                            println!("Error: THE INPUT IS EMPTY");
                            std::process::exit(1);
                        }
                        _ => {
                            println!("Error: INVALID PORT NUMBER");
                            std::process::exit(1);
                        }
                    },
                }
            } else {
                eprintln!("ERROR: INVALID PORT NUMBER");
                std::process::exit(1);
            }
        }
        
        "stop" | "STOP"=> {
            println!("HTTPX TERMINATED");
        }

        "view-ip" | "VIEW-IP" => {
            println!("IP ADDRESS: {}", get_ip());
        }

        "view-port" | "VIEW-PORT" => {
            println!("PORT NUMBER: {}", get_port());
        }

        "help" | "HELP" => {
            eprintln!("{}", HELP);
        }
        
        "-v" | "--version" => {
            eprintln!("HTTPX VERSION: {}", VERSION);
        }

        "config" | "CONFIG" => {
            let reader = std::io::BufReader::new(std::fs::File::open(kit::core::CONFIG_PATH).expect("Failed to open config.txt"));
            println!("\n\tHTTPX CONFIGURATION:\n");
            for line in reader.lines() {
                if let Ok(line) = line {
                    println!("\t{}", line);
                }
            }
        }
        _ => {
            eprintln!("ERROR: INVALID COMMAND\n{}", HELP);
            std::process::exit(1);
        }
    }    
}

fn main() {
    parse_args();
}