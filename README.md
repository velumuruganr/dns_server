# dns_server

Small recursive DNS resolver implementation in Rust.

## Overview

- Core modules:
  - [`buffer::BytePacketBuffer`](src/buffer.rs) — buffer + qname parsing/writing.  
  - [`header::DnsHeader`](src/header.rs) — DNS header read/write.  
  - [`question::QueryType`](src/question.rs), [`question::DnsQuestion`](src/question.rs) — question handling.  
  - [`record::DnsRecord`](src/record.rs) — DNS record read/write.  
  - [`packet::DnsPacket`](src/packet.rs) — packet-level parsing/serialization and helpers.  
  - Entrypoint and resolver logic (lookup loop): [`main::lookup`](src/main.rs), [`main::recursive_lookup`](src/main.rs), [`main::handle_query`](src/main.rs).

## Build

Requires Rust (stable).

```sh
cargo build --release
```

## Run

By default the server binds to UDP port 2053. Binding to port 53 requires elevated privileges; either run with appropriate permissions or change the port in [src/main.rs](src/main.rs).

```sh
# run (may need sudo depending on port)
cargo run --release
```

Example query (from another terminal):
```sh
dig @127.0.0.1 -p 2053 example.com A
```

## Notes & recommended improvements

- Consider extracting a typed error type instead of `Box<dyn Error>` and re-exporting `Result` from a dedicated module.
- Replace stdout prints with structured logging (`log` + `env_logger`) so library code can be quieter.
- Add unit tests for `buffer::BytePacketBuffer`, `record::DnsRecord` and `packet::DnsPacket` to validate parsing/write round-trips.
- Run `cargo fmt` and `cargo clippy` and address lints.
- See these files for quick fixes and hotspots:
  - `src/buffer.rs` — qname pointer handling and range checks.
  - `src/main.rs` — repeated socket binds, unwraps, and port choice.

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.