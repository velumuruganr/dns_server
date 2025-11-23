pub mod buffer;
pub mod header;
pub mod packet;
pub mod question;
pub mod record;
pub mod resolve;
pub mod server;

type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
