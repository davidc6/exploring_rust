mod ping;
pub use ping::Ping;

use crate::{Connection, Result};

pub enum Command {
    Ping(Ping),
}

impl Command {
    pub fn parse_cmd() -> std::result::Result<Command, ()> {
        Ok(Command::Ping(Ping::new()))
    }

    pub async fn run(self, conn: Connection) -> Result<()> {
        match self {
            Command::Ping(command) => command.respond(conn).await,
        }
    }
}
