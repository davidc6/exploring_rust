use crate::{Connection, Result};
pub use ping::Ping;
use std::io::Error;

mod ping;

pub enum Command {
    Ping(Ping),
}

impl Command {
    pub fn parse_cmd() -> std::result::Result<Command, Error> {
        Ok(Command::Ping(Ping::new()))
    }

    pub async fn run(self, conn: Connection) -> Result<()> {
        match self {
            Command::Ping(command) => command.respond(conn).await,
        }
    }
}
