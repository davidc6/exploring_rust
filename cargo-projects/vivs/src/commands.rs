use crate::data_chunk::DataChunk;
use crate::{Connection, Result};
// pub use ping::Ping;
use ping::Ping;
use std::io::Error;
use std::result::Result as NativeResult;

// mod ping;

pub mod ping;

// pub mod command {
pub enum Command {
    Ping(Ping),
    Unknown,
}

#[derive(Debug)]
pub enum ParseError {
    Other(crate::Error),
}

impl Command {
    // this method should parse a network frame/data unit instead of returning the hard-coded command
    // the frame essential is a redis command and for now this library will only support an array type
    // ref: https://redis.io/docs/reference/protocol-spec/#resp-arrays
    pub fn parse_cmd(frame: DataChunk) -> NativeResult<Command, ParseError> {
        let a = if let DataChunk::Array(arr) = frame {
            match arr.into_iter().next().unwrap() {
                DataChunk::Bulk(data) => data,
                _ => return Err("error".into()),
            }
        } else {
            return Err("incorrect".into());
        };

        let b = std::str::from_utf8(&a[..]).unwrap().to_lowercase();

        match &b[..] {
            "ping" => Ok(Command::Ping(Ping::new())),
            _ => Ok(Command::Unknown),
        }
        // match frame. into_iter()
        // TODO: remove hardcoded value here since we want to be able to process multiple commands (e.g. get, set, delete)
        // Ok(Command::Ping(Ping::new()))
    }

    // pub fn parse_command_from_frame() -> NativeResult<Command, Error> {}

    pub async fn run(self, conn: Connection) -> Result<()> {
        match self {
            Command::Ping(command) => command.respond(conn).await,
            Command::Unknown => Ok(()),
        }
    }
}
// }

impl From<String> for ParseError {
    fn from(src: String) -> ParseError {
        ParseError::Other(src.into())
    }
}

// impl std::error::Error for ParseError {}

impl From<&str> for ParseError {
    fn from(src: &str) -> ParseError {
        src.to_string().into()
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Other(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {}
