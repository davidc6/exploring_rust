use crate::{commands::ParseCommand, Command, Connection, DataStoreWrapper, Error};
use log::error;
use std::result::Result as NativeResult;

#[derive(Debug)]
pub enum HandlerError {
    CommandParsing(ParseCommand),
    Other(Error),
}

impl From<ParseCommand> for HandlerError {
    fn from(e: ParseCommand) -> Self {
        HandlerError::CommandParsing(e)
    }
}

impl From<Error> for HandlerError {
    fn from(e: Error) -> Self {
        HandlerError::Other(e)
    }
}

pub struct Handler {
    pub db: DataStoreWrapper,
    pub connection: Connection,
}

impl Handler {
    pub async fn run(&mut self) -> NativeResult<(), HandlerError> {
        // TODO: read a frame, should probably live in connection
        // read bits that host/client can send (frame)
        // this should return array of commands which will later parse
        let payload = self
            .connection
            .read_and_process_stream()
            .await
            .map_err(|e| {
                error!("Failed to process the stream: {}", e);
                e
            })?;

        // TODO get the command iterator
        // This will enable Command to get necessary data from the iterator by calling .next()
        let command = Command::parse_cmd(payload)?;
        command.run(&mut self.connection, &self.db).await?;

        Ok(())
    }
}
