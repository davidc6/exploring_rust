use crate::data_chunk::{DataChunk, DataChunkFrame};
use crate::{Connection, DataStoreWrapper, Result};
use get::Get;
use ping::Ping;
use set::Set;
use std::result::Result as NativeResult;

pub mod get;
pub mod ping;
pub mod set;

pub enum Command {
    Ping(Result<Ping>),
    Get(Get),
    Set(Set),
    Unknown,
}

pub enum DataType {
    SimpleString,
    Null,
    SimpleError,
}

#[derive(Debug)]
pub enum ParseError {
    Other(crate::Error),
}

impl Command {
    // This method should parse a network frame/data unit instead of returning the hard-coded command
    // the frame essential is a redis command and for now this library will only support an array type
    // ref: https://redis.io/docs/reference/protocol-spec/#resp-arrays
    pub fn parse_cmd(mut data_chunk: DataChunkFrame) -> NativeResult<Command, ParseError> {
        // The iterator should contain all the necessary commands and values i.e. [SET, key, value]
        // Next chunk should be of Bulk string type which should be the command we need to process
        let command = match data_chunk.next() {
            Ok(DataChunk::Bulk(data)) => data,
            _ => return Err("error".into()),
        };

        // To figure out which command needs to be processed,
        // we have to convert byte slice to a string slice that needs to be a valid UTF-8
        let command = std::str::from_utf8(&command).unwrap().to_lowercase();
        let command = match &command[..] {
            "ping" => Command::Ping(Ping::parse(data_chunk)),
            "get" => Command::Get(Get::parse(data_chunk).unwrap()),
            "set" => Command::Set(Set::parse(data_chunk).unwrap()),
            _ => Command::Unknown,
        };

        Ok(command)
    }

    pub async fn run(self, conn: Connection, db: DataStoreWrapper) -> Result<()> {
        match self {
            Command::Ping(command) => match command {
                Ok(command) => Ok(command.respond(conn).await?),
                Err(e) => Err(e),
            },
            Command::Get(command) => command.respond(conn, db).await,
            Command::Set(command) => command.respond(conn, db).await,
            Command::Unknown => Ok(()),
        }
    }
}

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

// impl std::fmt::Display for ParseError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ParseError::Other(err) => err.fmt(f),
//         }
//     }
// }

// impl std::error::Error for ParseError {}
