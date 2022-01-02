use std::net::TcpListener;
use std::io::Result;

fn main() {
    let ports = ["22", "3389", "8080", "80", "443", "8581"];

    for port in ports.iter() {
        match check_port(port) {
            Ok(_) => println!("\x1b[32m{:>5}: succeed\x1b[37m", port),
            Err(e) => println!("\x1b[31m{:>5}: fail\x1b[37m ({})", port, e),
        }
    }
}

fn check_port(port: &str) -> Result<TcpListener> {
    TcpListener::bind(format!("{}{}", "127.0.0.1:", port))
}

