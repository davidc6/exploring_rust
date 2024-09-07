#![deny(clippy::unwrap_in_result)]

use serde::Deserialize;
use std::{
    fmt::Display,
    sync::{LazyLock, OnceLock},
};
use tokio::net::TcpStream;

pub mod data_chunk;
use data_chunk::DataChunk;

pub mod listener;
pub use listener::Listener;

pub mod db;
pub use db::DataStore;

pub mod handler;
pub use handler::Handler;

pub mod connection;
pub use connection::Connection;

pub mod commands;
pub use commands::Command;

pub mod parser;
pub mod server;
pub mod utils;

use parser::Parser;

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

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    connection: ConnectionState,
}

#[derive(Deserialize, Debug, Default)]
struct ConnectionState {
    address: String,
    port: u16,
}

impl std::error::Error for Config {}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Could not load global config")
    }
}

// Load up the global config lazily
pub static VIVS_CONFIG_LAZY: LazyLock<Result<Config, String>> = LazyLock::new(|| {
    let Ok(config_dir) = std::env::current_dir() else {
        return Err("No current dir".to_owned());
    };

    let config_dir = config_dir.join("config/config.toml");
    let Ok(file_contents_as_string) = std::fs::read_to_string(config_dir) else {
        return Err("Could not read file".to_owned());
    };

    if let Ok(config) = toml::from_str(&file_contents_as_string) {
        config
    } else {
        let connection = ConnectionState {
            port: 9000,
            address: "127.0.0.1".to_owned(),
        };
        Ok(Config { connection })
    }
});

pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn new() -> GenericResult<Self> {
        let stream = TcpStream::connect(format!("127.0.0.1:{}", PORT)).await?;
        let connection = Connection::new(stream);
        Ok(Client { connection })
    }

    /// TODO: IMPROVEMENT
    ///
    /// Current implementation is very manual and basic.
    /// The idea is to eventually move to something like:
    /// client::command.set("get").set("key").conn(self.connection)?;
    pub async fn get(&mut self, value: String) -> Option<String> {
        // e.g. *2\r\n$3\r\GET\r\n$4\r\nMary\r\n
        let frame = format!("*2\r\n$3\r\nGET\r\n${}\r\n{}\r\n", value.len(), value);
        let _ = self.connection.write_complete_frame(&frame).await;

        let cursored_buffer = self.connection.process_stream().await;
        let data_chunk = DataChunk::read_chunk(&mut cursored_buffer.unwrap());
        let parser = Parser::new(data_chunk.unwrap());

        let Ok(mut parser) = parser else {
            return None;
        };

        match parser.next() {
            Some(DataChunk::Null) => None,
            Some(DataChunk::Bulk(b)) => Some(std::str::from_utf8(&b).unwrap().to_owned()),
            _ => None,
        }
    }

    /// TODO: IMPROVEMENT
    ///
    /// Same as get()
    pub async fn set(&mut self, key: String, value: String) -> String {
        // e.g. *3\r\n$3\r\nSET\r\n$4\r\nname\r\n$4\r\nMary\r\n
        let frame = format!(
            "*3\r\n$3\r\nSET\r\n${}\r\n{}\r\n${}\r\n{}\r\n",
            key.len(),
            key,
            value.len(),
            value
        );
        let _ = self.connection.write_complete_frame(&frame).await;

        let mut data_chunk = self.connection.process_stream().await.unwrap();
        let data_chunk = Parser::new(DataChunk::read_chunk(&mut data_chunk).unwrap());
        let frame = DataChunk::read_chunk_frame(&mut data_chunk.unwrap())
            .await
            .unwrap();
        let frame_utf8 = std::str::from_utf8(&frame);
        frame_utf8.unwrap().to_owned()
    }

    pub async fn delete(&mut self, key: String) -> String {
        let frame = format!("*2\r\n$6\r\nDELETE\r\n${}\r\n{}\r\n", key.len(), key);
        let _ = self.connection.write_complete_frame(&frame).await;

        let frame = self.connection.process_stream().await;
        let processed_stream = DataChunk::read_chunk(&mut frame.unwrap());
        let frame = Parser::new(processed_stream.unwrap());

        if let Ok(mut frame) = frame {
            return match frame.next() {
                Some(DataChunk::Null) => "0".to_owned(),
                Some(DataChunk::Integer(val)) => {
                    // convert Bytes to bytes array
                    // then determine endianness to create u64 integer value from the bytes array
                    // and return integer as string
                    let bytes_slice = val.slice(0..8);

                    // converts the slice to an array of u8 elements (since u64 is 8 bytes)
                    let arr_u8: [u8; 8] = bytes_slice[0..8].try_into().unwrap();

                    if cfg!(target_endian = "big") {
                        u64::from_be_bytes(arr_u8)
                    } else {
                        u64::from_le_bytes(arr_u8)
                    }
                    .to_string()
                }
                _ => "0".to_owned(),
            };
        }

        "0".to_owned()
    }
}
