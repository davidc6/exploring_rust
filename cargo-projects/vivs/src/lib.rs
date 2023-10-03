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
//
// Send - safe to send to another thread
// Sync - safe to share between threads (A type can be Sync only if it is Send)
//
// Multiple trait bounds here are applied with a "+" in order
// for Error to be bound by them
//
// <Box> returns reference to some memory on the heap,
// since Error can be type is only known at runtime.
//
// dyn highlights the fact that calls to methods on the associated trait
// (Error, in this case) are dynamically dispatched
pub type Error = Box<dyn std::error::Error + Send + Sync>;
// we use type alias here to to aviod having to repeat the Error type
// For example, Result<bool> is interpreted as Result<bool, Error>
pub type Result<T> = std::result::Result<T, Error>;

// TODO: investigate and remove
// impl Error {
//     fn new(m: &str) -> Error {
//         Error()
//     }
// }

// impl From<CommandErrors> for Error {
//     fn from(err: CommandErrors) -> Self {
//         Error::new(err.description())
//     }
// }
