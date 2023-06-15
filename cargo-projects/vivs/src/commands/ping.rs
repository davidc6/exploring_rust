use crate::{
    data_chunk::{DataChunk, DataChunkFrame},
    Connection, Result,
};
use bytes::Buf;

#[derive(Debug, Default)]
pub struct Ping {
    message: Option<DataChunk>,
}

impl Ping {
    pub fn new(message: Option<DataChunk>) -> Self {
        Ping { message }
    }

    pub fn parse(mut data: DataChunkFrame) -> Result<Self> {
        // TODO: peek instead of next so that we can
        match data.next() {
            Ok(data) => Ok(Ping::new(Some(data))),
            Err(_) => Ok(Ping::default()),
            // Err(e) => Err(e.into()),
        }
    }

    pub async fn respond(self, conn: Connection) -> Result<()> {
        let default_response = "PONG";

        // write message that was "pinged" or "pong" back if no message was found
        if let Some(message) = self.message {
            match message {
                DataChunk::Bulk(message) => conn.write_chunk(message.chunk()).await?,
                _ => conn.write_chunk(default_response.as_bytes()).await?,
            }

            Ok(())
        } else {
            conn.write_chunk(default_response.as_bytes()).await?;

            Ok(())
        }
    }
}
