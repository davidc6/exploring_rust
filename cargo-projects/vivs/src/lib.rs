// include the code found in src/listener.rs
pub mod listener;
// use declarations are private to the containing module
// hence we use pub keyword to re-export a name
pub use listener::Listener;

pub mod db;
pub use db::DataStore;
pub use db::DataStoreWrapper;

pub mod server;

pub mod handler;
pub use handler::Handler;

pub mod connection;
pub use connection::Connection;

pub mod commands;
pub use commands::Command;

pub mod data_chunk;

// any error that is safe to pass between threads (Send + Sync)
// Send - safe to send to another thread
// Sync - safe to share between threads (A type can be Sync only if it is Send)
pub type Error = Box<dyn std::error::Error + Send + Sync>;
// we use type alias here to to aviod having to repeat
// the Error type
// For example, Result<bool> is interpreted as Result<bool, Error>
pub type Result<T> = std::result::Result<T, Error>;
