use crate::{data_chunk::DataChunkFrame, Connection, Result};
use bytes::Bytes;
use log::info;

const PING_CMD: &str = "PING";

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
        if let Some(message) = self.message {
            info!("{}", format!("{:?} {}", PING_CMD, message));
            conn.write_chunk(super::DataType::SimpleString, Some(message.as_bytes()))
                .await?;
        } else {
            info!("{}", format!("{:?}", PING_CMD));
            conn.write_chunk(super::DataType::SimpleString, Some(b"PONG"))
                .await?
        }
        Ok(())
    }

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
