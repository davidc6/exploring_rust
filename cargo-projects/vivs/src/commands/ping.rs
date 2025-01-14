use crate::{parser::Parser, Connection, GenericResult};
use bytes::Bytes;
use log::info;

pub const PING_CMD: &str = "ping";
pub const PONG: &str = "PONG";

#[derive(Debug, Default)]
pub struct Ping {
    message: Option<String>,
}

impl Ping {
    pub fn new(message: Option<String>) -> Self {
        Ping { message }
    }

    pub fn parse(mut data: Parser) -> Self {
        match data.next_as_str() {
            Ok(value) => Ping::new(value),
            Err(_) => Ping::default(),
        }
    }

    pub async fn respond(self, conn: &mut Connection) -> GenericResult<()> {
        if let Some(message) = self.message {
            info!(
                "{}",
                format!(
                    "{:?} {:?} {:?}",
                    conn.connected_peer_addr(),
                    PING_CMD.to_uppercase(),
                    message
                )
            );
            conn.write_chunk(super::DataType::SimpleString, message.as_bytes())
                .await?;
        } else {
            info!(
                "{:?} {:?}",
                conn.connected_peer_addr(),
                PING_CMD.to_uppercase()
            );
            conn.write_chunk(super::DataType::SimpleString, b"PONG")
                .await?;
        }
        Ok(())
    }

    /// Pushes optional PING [message] to the segments array if it exists.
    /// In order to do this, a default Parser gets created which
    /// takes is a command first and then the optional message.
    /// This is a bit of a hack since Parser and DataChunk are
    /// different structs (even though potentially get could be one in the future).
    pub fn into_chunk(self) -> Parser {
        let data_chunk_frame = Parser::default();
        let cmd = format!("{}\r\n", PING_CMD);
        let mut data_chunk_frame = data_chunk_frame.push_bulk_str(Bytes::from(cmd));

        if let Some(msg) = self.message {
            data_chunk_frame = data_chunk_frame.push_bulk_str(format!("{}\r\n", msg).into());
        }

        data_chunk_frame
    }
}
