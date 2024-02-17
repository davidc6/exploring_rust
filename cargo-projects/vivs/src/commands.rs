use crate::data_chunk::{DataChunkError, DataChunkFrame};
use crate::utils::unknown_cmd_err;
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
    Unknown(String),
}

pub enum DataType {
    SimpleString,
    Null,
    SimpleError,
    Integer,
}

#[derive(Debug)]
pub enum ParseCommand {
    Other(crate::Error),
    Unknown,
    NoCommand,
}

impl From<Error> for ParseCommand {
    fn from(error: Error) -> Self {
        ParseCommand::Other(error)
    }
}

impl From<DataChunkError> for ParseCommand {
    fn from(error: DataChunkError) -> Self {
        ParseCommand::Other(Box::new(error))
    }
}

impl Command {
    pub fn parse_cmd(mut data_chunk: DataChunkFrame) -> NativeResult<Command, ParseCommand> {
        // The iterator should contain all the necessary commands and values e.g. [SET, key, value]
        // The first value is the command itself
        let Some(command) = data_chunk.next_as_str()?.map(|val| val.to_lowercase()) else {
            return Err(ParseCommand::NoCommand);
        };

        // To figure out which command needs to be processed,
        // we have to convert byte slice to a string slice that needs to be a valid UTF-8
        let command = match &command[..] {
            "ping" => Command::Ping(Ping::parse(data_chunk)),
            "get" => Command::Get(Get::parse(data_chunk)),
            "set" => Command::Set(Set::parse(data_chunk)),
            "delete" => Command::Delete(Delete::parse(data_chunk)),
            val => Command::Unknown(val.to_owned()),
        };

        Ok(command)
    }

    pub async fn run(self, conn: &mut Connection, db: &DataStoreWrapper) -> Result<()> {
        match self {
            Command::Ping(command) => command.respond(conn).await,
            Command::Get(command) => command.respond(conn, db).await,
            Command::Set(command) => command.respond(conn, db).await,
            Command::Delete(command) => command.respond(conn, db).await,
            Command::Unknown(command) => {
                conn.write_error(unknown_cmd_err(command).as_bytes())
                    .await?;
                Ok(())
            }
        }
    }
}

impl From<String> for ParseCommand {
    fn from(src: String) -> ParseCommand {
        ParseCommand::Other(src.into())
    }
}

impl From<&str> for ParseCommand {
    fn from(src: &str) -> ParseCommand {
        src.to_string().into()
    }
}
