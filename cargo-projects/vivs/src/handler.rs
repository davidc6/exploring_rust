use crate::{Command, Connection, DataStoreWrapper, Result};

pub struct Handler {
    pub db: DataStoreWrapper,
    pub tcp_connection: Connection,
}

impl Handler {
    pub async fn run(self) -> Result<()> {
        println!("Hello");

        let cmd = Command::parse_cmd().unwrap();
        // TODO pass db
        cmd.run(self.tcp_connection).await?;

        Ok(())
    }
}
