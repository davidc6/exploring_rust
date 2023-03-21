use crate::{Command, Connection, DataStoreWrapper, Result};

pub struct Handler {
    pub db: DataStoreWrapper,
    pub connection: Connection,
}

impl Handler {
    pub async fn run(self) -> Result<()> {
        println!("Hello");

        // TODO: read a frame, should probably live in connection

        // TODO get the command interator
        let command = Command::parse_cmd().unwrap();

        // TODO pass db
        command.run(self.connection).await?;

        Ok(())
    }
}
