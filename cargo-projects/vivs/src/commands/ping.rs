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
        let resp_val = "PONG";

        if let Some(message) = self.message {
            match message {
                DataChunk::Bulk(data) => conn.write_chunk(data.chunk()).await?,
                _ => conn.write_chunk(resp_val.as_bytes()).await?,
            }

            Ok(())
        } else {
            conn.write_chunk(resp_val.as_bytes()).await?;

            Ok(())
        }
    }
}
