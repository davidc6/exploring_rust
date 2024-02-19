use crate::data_chunk::{DataChunkError, DataChunkFrame};
use crate::utils::{unknown_cmd_err, NO_CMD_ERR};
use crate::{connection, Connection, DataStore, GenericError, GenericResult};
use delete::Delete;
use get::Get;
use ping::Ping;
use set::Set;
use self::delete::DELETE_CMD;
use self::get::GET_CMD;
use self::ping::PING_CMD;
use self::set::SET_CMD;

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
    None,
}

pub enum DataType {
    SimpleString,
    Null,
    SimpleError,
    Integer,
}

#[derive(Debug)]
pub enum ParseCommandErr {
    Other(crate::GenericError),
    Unknown,
    NoCommand,
}

impl From<GenericError> for ParseCommandErr {
    fn from(error: GenericError) -> Self {
        ParseCommandErr::Other(error)
    }
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
    fn parse(data: DataChunkFrame) -> Self;
    fn respond(
        &self,
        connection: &mut Connection,
        datastore: &DataStore,
    ) -> impl std::future::Future<Output = GenericResult<()>> + Send;
}

impl Command {
    pub fn parse_cmd(mut data_chunk: DataChunkFrame) -> Result<Command, ParseCommandErr> {
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
            Command::None => {
                conn.write_error(NO_CMD_ERR.as_bytes()).await?;
                Ok(())
            }
            Command::Unknown(command) => {
                conn.write_error(unknown_cmd_err(command).as_bytes())
                    .await?;
                Ok(())
            }
        }
    }
}
