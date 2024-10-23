use self::delete::DELETE_CMD;
use self::get::GET_CMD;
use self::ping::PING_CMD;
use self::set::SET_CMD;
use self::ttl::TTL_CMD;
use crate::data_chunk::DataChunkError;
use crate::parser::Parser;
use crate::utils::{unknown_cmd_err, NO_CMD_ERR};
use crate::{ClusterInstanceConfig, Connection, DataStore, GenericResult};
use asking::Ask;
use core::str;
use delete::Delete;
use get::Get;
use ping::Ping;
use set::Set;
use std::env::current_dir;
use tokio::fs;
use ttl::Ttl;

pub mod asking;
pub mod delete;
pub mod get;
pub mod ping;
pub mod set;
pub mod ttl;

#[derive(Debug)]
pub enum Command {
    Ping(Ping),
    Get(Get),
    Set(Set),
    Delete(Delete),
    Ttl(Ttl),
    Ask(Ask),
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

pub trait AskCommand {
    async fn check_ask(&self, key: &str, conn: &mut Connection) -> Option<(u16, String)> {
        // check if config exists, we'll most likely need to store it in memory to avoid constant IO (?)
        let own_addr = conn.own_addr().unwrap().to_string();

        let port = own_addr.split(":").nth(1).unwrap_or_default();
        let current_dir = current_dir().unwrap();
        let node_config = format!("{}/{}.toml", current_dir.display(), port);

        // When no cluster (<port>.conf) file is found,
        // We can assume that Vivs is not running in the cluster mode.
        // Therefore normal processing of incoming command should take place.
        let Ok(file_contents) = fs::read(node_config).await else {
            return None;
        };

        let file_contents = str::from_utf8(&file_contents[0..]).unwrap();
        let nodes = toml::from_str::<ClusterInstanceConfig>(file_contents).unwrap();

        // Work out a cell / hash slot
        const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
        let key_hash = X25.checksum(key.as_bytes()) % 16384;

        // Iterate over all current nodes in the cluster
        for (ip, config) in nodes {
            let cell_range = config.position.0..config.position.1;

            let is_in_range = cell_range.contains(&key_hash.into());
            if own_addr == ip && is_in_range {
                return None;
            }

            if is_in_range {
                return Some((key_hash, ip));
            }
        }

        None
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
    fn process_location(a: Command) -> Command {
        Command::Ask(Ask {
            command: Box::new(a),
        })
    }

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
            SET_CMD => Self::process_location(Command::Set(Set::parse(data_chunk))),
            DELETE_CMD => Command::Delete(Delete::parse(data_chunk)),
            TTL_CMD => Command::Ttl(Ttl::parse(data_chunk)),
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
