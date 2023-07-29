use crate::data_chunk::{DataChunk, DataChunkFrame};
use crate::Result;
use crate::{Connection, DataStoreWrapper};
use bytes::Buf;
use std::fmt::Display;

#[derive(Debug)]
enum GetError {
    NoKey,
}

impl std::error::Error for GetError {}

impl Display for GetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

pub struct Get {
    key: Option<DataChunk>,
}

impl Get {
    pub fn parse(mut data: DataChunkFrame) -> Result<Self> {
        // TODO: peek instead of next so that we can
        // TODO: should this even be a Result?
        match data.next() {
            Ok(data) => Ok(Get { key: Some(data) }),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn respond(self, conn: Connection, db: DataStoreWrapper) -> Result<()> {
        let Some(key) = self.key else {
            return Err(Box::new(GetError::NoKey));
        };

        match key {
            DataChunk::Bulk(key) => {
                let key = std::str::from_utf8(key.chunk())?;
                if key.is_empty() {
                    let error = "wrong number of arguments";
                    conn.write_error(error.as_bytes()).await?;
                    return Ok(());
                }

                let data_store_guard = db.db.read().await;

                // TODO: once TTL is figured out, check for expiration here
                if let Some(value) = data_store_guard.db.get(key) {
                    conn.write_chunk(super::DataType::SimpleString, Some(value.as_bytes()))
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
