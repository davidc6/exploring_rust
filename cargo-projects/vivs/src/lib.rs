pub mod db;
pub use db::DataStore;
pub use db::DataStoreWrapper;

pub mod server;

// include the code found in src/listener.rs
pub mod listener;
// use declarations are private to the containing module
// hence we use pub keyword to re-export a name
pub use listener::Listener;

pub mod handler;
pub use handler::Handler;

pub mod connection;
pub use connection::Connection;

pub mod commands;
pub use commands::Command;

pub mod data_chunk;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
