use crate::{Command, Connection, DataStoreWrapper, Result};

pub struct Handler {
    pub db: DataStoreWrapper,
    pub connection: Connection,
}

impl Handler {
    pub async fn run(mut self) -> Result<()> {
        // TODO: read a frame, should probably live in connection
        // read bits that host/client can send (frame)
        // this should return array of commands which will later parse
        let payload = self.connection.read_and_process_stream().await?;

        // TODO get the command interator
        // This will enable Command to get necessary data from the iterator by calling .next()
        let command = Command::parse_cmd(payload).unwrap();

        // TODO pass db
        command.run(self.connection).await?;

        Ok(())
    }
}
