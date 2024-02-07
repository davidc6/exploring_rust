use crate::{data_chunk::DataChunkFrame, Connection, Result};
use bytes::Bytes;
use log::info;

const PING_CMD: &str = "PING";
pub const PONG: &str = "PONG";

#[derive(Debug, Default)]
pub struct Ping {
    message: Option<String>,
}

impl Ping {
    pub fn new(message: Option<String>) -> Self {
        Ping { message }
    }

    pub fn parse(mut data: DataChunkFrame) -> Result<Self> {
        match data.next_as_str() {
            Ok(value) => Ok(Ping::new(Some(value))),
            Err(_) => Ok(Ping::default()),
        }
    }

    pub async fn respond(self, conn: &mut Connection) -> Result<()> {
        if let Some(mut message) = self.message {
            // This is also a hack similar to read_chunk_frame() in the connection.rs module
            if message.chars().nth(0) != Some('\"') {
                message = format!("\"{}\"", message);
            }

            info!(
                "{}",
                format!(
                    "{:?} {:?} {}",
                    conn.connected_peer_addr(),
                    PING_CMD,
                    message
                )
            );
            conn.write_chunk(super::DataType::SimpleString, Some(message.as_bytes()))
                .await?;
        } else {
            info!(
                "{:?} {}",
                conn.connected_peer_addr(),
                format!("{:?}", PING_CMD)
            );
            conn.write_chunk(super::DataType::SimpleString, Some(b"PONG"))
                .await?
        }
        Ok(())
    }

    /// Pushes optional PING [message] to the segments array if it exists.
    /// In order to do this, a default DataChunkFrame gets created which
    /// takes is a command first and then the optional message.
    /// This is a bit of a hack since DataChunkFrame and DataChunk are
    /// different structs (even though potentially get could be one in the future).
    pub fn into_chunk(self) -> DataChunkFrame {
        let data_chunk_frame = DataChunkFrame::default();
        let cmd = format!("{}\r\n", PING_CMD);
        let mut data_chunk_frame = data_chunk_frame.push_bulk_str(Bytes::from(cmd));

        if let Some(msg) = self.message {
            data_chunk_frame = data_chunk_frame.push_bulk_str(format!("{}\r\n", msg).into());
        }

        data_chunk_frame
    }
}
