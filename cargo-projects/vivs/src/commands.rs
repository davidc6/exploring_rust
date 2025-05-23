use self::delete::DELETE_CMD;
use self::get::GET_CMD;
use self::ping::PING_CMD;
use self::set::SET_CMD;
use self::ttl::TTL_CMD;
use crate::data_chunk::DataChunkError;
use crate::parser::Parser;
use crate::{Connection, DataStore, GenericResult};
use ask::{Ask, ASK_CMD};
use asking::{Asking, ASKING_CMD};
use core::str;
use delete::Delete;
use get::Get;
use ping::Ping;
use set::Set;
use ttl::Ttl;

pub mod ask;
pub mod asking;
pub mod delete;
pub mod get;
pub mod ping;
pub mod set;
pub mod ttl;

pub const FALSE_CMD: &str = "FALSECMD";

pub const NO_CMD_ERR: &str = "No command supplied\r\n";
pub const NO_CMD: &str = "NOCMD";

pub const INCORRECT_ARGS_ERR: &str = "Incorrect number of arguments\r\n";
pub const ARGS_NUM: &str = "ARGSNUM";

pub const VALUE_NOT_INT_ERR: &str = "Value is not an integer\r\n";
pub const NON_INT: &str = "NONINT";

#[derive(Debug)]
pub enum Command {
    Ping(Ping),
    Get(Get),
    Set(Set),
    Delete(Delete),
    Ttl(Ttl),
    Ask(Ask),
    Unknown(String),
    Asking(Asking),
    None,
}

pub enum DataType {
    SimpleString,
    Null,
    SimpleError,
    Integer,
    BulkString,
    Array,
}

#[derive(Debug)]
pub enum ParseCommandErr {
    Other(crate::GenericError),
    Unknown,
}

impl From<DataChunkError> for ParseCommandErr {
    fn from(error: DataChunkError) -> Self {
        ParseCommandErr::Other(Box::new(error))
    }
}

impl From<String> for ParseCommandErr {
    fn from(src: String) -> ParseCommandErr {
        ParseCommandErr::Other(src.into())
    }
}

impl From<&str> for ParseCommandErr {
    fn from(src: &str) -> ParseCommandErr {
        src.to_string().into()
    }
}

pub trait CommonCommand {
    fn parse(data: Parser) -> Self;
    fn respond(
        &self,
        connection: &mut Connection,
        datastore: &DataStore,
    ) -> impl std::future::Future<Output = GenericResult<()>> + Send;
}

impl Command {
    pub fn parse_cmd(mut data_chunk: Parser) -> Result<Command, ParseCommandErr> {
        // The iterator should contain all the necessary commands and values e.g. [SET, key, value]
        // The first value is the command itself
        let Some(command) = data_chunk.next_as_str()?.map(|val| val.to_lowercase()) else {
            return Ok(Command::None);
        };

        // To figure out which command needs to be processed,
        // we have to convert byte slice to a string slice that needs to be a valid UTF-8
        let command = match &command[..] {
            PING_CMD => Command::Ping(Ping::parse(data_chunk)),
            GET_CMD => Command::Get(Get::parse(data_chunk)),
            SET_CMD => Command::Set(Set::parse(data_chunk)),
            DELETE_CMD => Command::Delete(Delete::parse(data_chunk)),
            TTL_CMD => Command::Ttl(Ttl::parse(data_chunk)),
            ASK_CMD => Command::Ask(Ask::parse()),
            ASKING_CMD => Command::Asking(Asking::parse(data_chunk)),
            "" => Command::None,
            val => Command::Unknown(val.to_owned()),
        };

        Ok(command)
    }

    pub async fn run(self, conn: &mut Connection, db: &DataStore) -> GenericResult<()> {
        match self {
            Command::Ping(command) => command.respond(conn).await,
            Command::Get(command) => command.respond(conn, db).await,
            Command::Set(command) => command.respond(conn, db).await,
            Command::Delete(command) => command.respond(conn, db).await,
            Command::Ttl(command) => command.respond(conn, db).await,
            Command::Ask(command) => command.respond(conn).await,
            Command::Asking(command) => command.respond(conn).await,
            Command::None => {
                conn.write_error(NO_CMD.as_bytes()).await?;
                Ok(())
            }
            Command::Unknown(command) => {
                let error_msg = format!("Unknown command {:?}", command);
                conn.write_error_with_msg(FALSE_CMD.as_bytes(), error_msg.as_bytes())
                    .await?;
                Ok(())
            }
        }
    }
}
