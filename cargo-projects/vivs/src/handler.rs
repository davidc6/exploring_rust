use crate::{commands::ParseCommandErr, Command, Connection, DataStore, GenericError};

#[derive(Debug)]
pub enum HandlerError {
    CommandParsing(ParseCommandErr),
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

pub struct Handler {
    pub db: DataStore,
    pub connection: Connection,
}

impl Handler {
    pub async fn run(&mut self) -> Result<(), HandlerError> {
        // TODO: should probably have a separate parser module
        // read a frame, should probably live in connection
        // read bits that host/client can send (frame)
        // this should return array of commands which will later parse
        let payload = self.connection.read_and_process_stream().await?;

        let command = Command::parse_cmd(payload)?;
        command.run(&mut self.connection, &self.db).await?;

        Ok(())
    }
}
