use crate::{Connection, Result};
pub use ping::Ping;
use std::io::Error;
use std::result::Result as NativeResult;

mod ping;

pub enum Command {
    Ping(Ping),
}

impl Command {
    // this method should parse a network frame/data unit instead of returning the hard-coded command
    // the frame essential is a redis command and for now this library will only support an array type
    // ref: https://redis.io/docs/reference/protocol-spec/#resp-arrays
    pub fn parse_cmd() -> NativeResult<Command, Error> {
        Ok(Command::Ping(Ping::new()))
    }

    // pub fn parse_command_from_frame() -> NativeResult<Command, Error> {}

    pub async fn run(self, conn: Connection) -> Result<()> {
        match self {
            Command::Ping(command) => command.respond(conn).await,
        }
    }
}
