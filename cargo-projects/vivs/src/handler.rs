use crate::data_chunk::DataChunkError;
use crate::{
    commands::ParseCommandErr, data_chunk::DataChunk, parser::Parser, Connection, DataStore,
    GenericError,
};

use crate::commands::Command;

#[derive(Debug)]
pub enum HandlerError {
    CommandParsing(ParseCommandErr),
    DataChunk(DataChunkError),
    Other(GenericError),
}

impl From<ParseCommandErr> for HandlerError {
    fn from(e: ParseCommandErr) -> Self {
        HandlerError::CommandParsing(e)
    }
}

impl From<GenericError> for HandlerError {
    fn from(e: GenericError) -> Self {
        HandlerError::Other(e)
    }
}

impl From<DataChunkError> for HandlerError {
    fn from(e: DataChunkError) -> Self {
        HandlerError::DataChunk(e)
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

    pub async fn run(&mut self) -> Result<(), HandlerError> {
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
