use dns_server::dns::server;

mod common;

#[test]
fn test_dns_server() {
    common::setup();

    let result = server::run_server();
    assert!(result.is_ok());
}
