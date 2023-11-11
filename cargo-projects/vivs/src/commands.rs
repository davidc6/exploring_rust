use crate::data_chunk::{DataChunkFrame, Error as DataChunkError};
use crate::{Connection, DataStoreWrapper, Error, Result};
use delete::Delete;
use get::Get;
use ping::Ping;
use set::Set;
use std::result::Result as NativeResult;

pub mod delete;
pub mod get;
pub mod ping;
pub mod set;

pub enum Command {
    Ping(Ping),
    Get(Get),
    Set(Set),
    Delete(Delete),
    Unknown,
}

pub enum DataType {
    SimpleString,
    Null,
    SimpleError,
    Integer,
}

#[derive(Debug)]
pub enum ParseError {
    Other(crate::Error),
}

impl From<Error> for ParseError {
    fn from(error: Error) -> Self {
        ParseError::Other(error)
    }
}

impl From<DataChunkError> for ParseError {
    fn from(error: DataChunkError) -> Self {
        ParseError::Other(Box::new(error))
    }
}

impl Command {
    // This method should parse a network frame/data unit instead of returning the hard-coded command
    // the frame essential is a redis command and for now this library will only support an array type
    // ref: https://redis.io/docs/reference/protocol-spec/#resp-arrays
    pub fn parse_cmd(mut data_chunk: DataChunkFrame) -> NativeResult<Command, ParseError> {
        // The iterator should contain all the necessary commands and values i.e. [SET, key, value]
        // .next() chunk should be of Bulk string type which should be the command we need to process
        let command = data_chunk.next_as_str()?.to_lowercase();

        // TODO: remove unwraps
        // To figure out which command needs to be processed,
        // we have to convert byte slice to a string slice that needs to be a valid UTF-8
        let command = match &command[..] {
            "ping" => Command::Ping(Ping::parse(data_chunk)?),
            "get" => Command::Get(Get::parse(data_chunk)?),
            "set" => Command::Set(Set::parse(data_chunk).unwrap()),
            "delete" => Command::Delete(Delete::parse(data_chunk)?),
            _ => Command::Unknown,
        };

        Ok(command)
    }

    pub async fn run(self, conn: &mut Connection, db: &DataStoreWrapper) -> Result<()> {
        match self {
            Command::Ping(command) => command.respond(conn).await,
            Command::Get(command) => command.respond(conn, db).await,
            Command::Set(command) => command.respond(conn, db).await,
            Command::Delete(command) => command.respond(conn, db).await,
            // Error?
            Command::Unknown => Ok(()),
        }
    }
}

impl From<String> for ParseError {
    fn from(src: String) -> ParseError {
        ParseError::Other(src.into())
    }
}

impl From<&str> for ParseError {
    fn from(src: &str) -> ParseError {
        src.to_string().into()
    }
}
