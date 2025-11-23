use dns_server::dns::server::run_server;

fn main() {
    if let Err(e) = run_server() {
        eprintln!("Server error: {}", e);
    }
}
