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
    pub fn parse_cmd(data_chunk: DataChunk) -> NativeResult<Command, ParseError> {
        let commands = if let DataChunk::Array(arr) = data_chunk {
            match arr.into_iter().next().unwrap() {
                DataChunk::Bulk(data) => data,
                _ => return Err("error".into()),
            }
        } else {
            return Err("incorrect".into());
        };

        // To figure out which command needs to be processed,
        // we have to convert slice of bytes to a string slice
        let command = std::str::from_utf8(&commands).unwrap().to_lowercase();
        let command = match &command[..] {
            "ping" => Command::Ping(Ping::new()),
            _ => Command::Unknown,
        };

        Ok(command)
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