use dns_server::dns::server;

fn main() {
    if let Err(e) = server::run_server() {
        eprintln!("Server error: {}", e);
    }
}
