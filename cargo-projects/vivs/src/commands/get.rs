use crate::data_chunk::{DataChunk, DataChunkFrame};
use crate::Result;
use crate::{Connection, DataStoreWrapper};
use bytes::Buf;

pub struct Get {
    key: Option<DataChunk>,
}

impl Get {
    pub fn parse(mut data: DataChunkFrame) -> Result<Self> {
        // TODO: peek instead of next so that we can
        match data.next() {
            Ok(data) => Ok(Get { key: Some(data) }),
            Err(e) => Err(e.into()), 
        }
    }

    pub async fn respond(self, conn: Connection, db: DataStoreWrapper) -> Result<()> {
        match self.key.unwrap() {
            DataChunk::Bulk(key) => {
                let key = std::str::from_utf8(key.chunk());

                if key.is_err() {
                    return Err("Failed to convert to UTF8".into());
                }

                let key = key.unwrap();

                if key.is_empty() {
                    // TODO: extract various error prefix types
                    let error = "ERROR wrong number of arguments".to_owned();
                    conn.write_chunk(super::DataType::SimpleError, Some(error.as_bytes()))
                        .await?;
                    return Ok(());
                }

                let data_store_guard = db.db.read().await;

                // TODO: once TTL is figured out, check for expiration here
                if let Some(value) = data_store_guard.db.get(key) {
                    conn.write_chunk(super::DataType::Null, Some(value.as_bytes()))
                        .await?
                } else {
                    conn.write_chunk(super::DataType::Null, None).await?
                }
            }
            _ => {
                // TODO - to rethink
                panic!("Error")
            }
        }

        Ok(())
    }
}
