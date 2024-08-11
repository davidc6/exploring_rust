use crate::commands::{Command, ParseCommandErr};
use crate::data_chunk::{DataChunk, DataChunkError};
use crate::{parser::Parser, Connection, DataStore, GenericError};
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub enum HandlerError {
    CommandParsing(ParseCommandErr),
    DataChunk(DataChunkError),
    ClientDisconnected,
    Other(GenericError),
}

impl Display for HandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            HandlerError::ClientDisconnected => write!(f, "Client disconnected"),
            HandlerError::CommandParsing(err) => write!(f, "Failed when parsing {:?}", err),
            HandlerError::DataChunk(err) => write!(f, "Data chunk error: {}", err),
            HandlerError::Other(err) => write!(f, "Error: {}", err),
        }
    }
}

impl From<ParseCommandErr> for HandlerError {
    fn from(e: ParseCommandErr) -> Self {
        Self::CommandParsing(e)
    }
}

impl From<GenericError> for HandlerError {
    fn from(e: GenericError) -> Self {
        Self::Other(e)
    }
}

impl From<DataChunkError> for HandlerError {
    fn from(e: DataChunkError) -> Self {
        match e {
            DataChunkError::NoBytesRemaining => HandlerError::ClientDisconnected,
            _ => HandlerError::DataChunk(e),
        }
    }
}

pub struct Handler {
    pub db: DataStore,
    pub connection: Connection,
}

impl Handler {
    pub fn new(db: DataStore, connection: Connection) -> Self {
        Handler { db, connection }
    }

    pub async fn run(&mut self) -> std::result::Result<(), HandlerError> {
        // TODO: should probably have a separate parser module
        // read a frame, should probably live in connection
        // read bits that host/client can send (frame)
        // this should return array of commands which will later parse
        let mut cursored_buffer = self.connection.process_stream().await?;

        // read chunk
        let data = DataChunk::read_chunk(&mut cursored_buffer);
        let data = Parser::new(data?);

        let command = Command::parse_cmd(data?)?;
        command.run(&mut self.connection, &self.db).await?;

        Ok(())
    }
}
