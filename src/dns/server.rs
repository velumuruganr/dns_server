use super::Result;
use super::resolve;
use std::net::UdpSocket;

pub fn run_server() -> Result<()> {
    let socket = UdpSocket::bind(("0.0.0.0", 2053))?;

    loop {
        if let Err(e) = resolve::handle_query(&socket) {
            eprintln!("An error occured: {}", e);
        }
    }
}
