// include the code found in src/listener.rs
pub mod listener;
// use declarations are private to the containing module
// hence we use pub keyword to re-export a name
pub use listener::Listener;

pub mod db;
pub use db::DataStore;

pub mod server;

pub mod handler;
pub use handler::Handler;

pub mod connection;
pub use connection::Connection;

pub mod commands;
pub use commands::Command;

pub mod data_chunk;

pub mod utils;

// Boxing errors is a good starting point but would need to be reconsidered.
//
// Any error that is safe to pass between threads implements Send + Sync marker traits.
// Send - safe to send to another thread
// Sync - safe to share between threads (A type can be Sync only if it is Send)
//
// Multiple trait bounds here are applied with a "+" in order
// for Error to be bound by them.
//
// <Box> returns reference to some memory on the heap,
// since Error can be of a type only known at runtime.
// These are unsized in Rust terminology i.e. can have a different
// size in memory.
//
// dyn highlights the fact that calls to methods on the associated trait
// (Error, in this case) are dynamically dispatched.
//
// This is a severely type-erased error type which only reveals that it's an error
// without an ability to introspect it.
pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

// We use type alias here to to avoid having to repeat the Error type
// For example, Result<bool> is interpreted as Result<bool, Error>
pub type GenericResult<T> = std::result::Result<T, GenericError>;

// This is the default port the server listens on
pub const PORT: u16 = 9000;
