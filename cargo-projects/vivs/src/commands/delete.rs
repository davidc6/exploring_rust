use std::fmt::Display;

use crate::{data_chunk::DataChunkFrame, Connection, DataStoreWrapper, Result};

#[derive(Debug)]
enum DeleteError {
    NoKey,
}

impl std::error::Error for DeleteError {}

impl Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeleteError::NoKey => write!(f, "No key was passed to GET command"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Delete {
    key: Option<String>,
}

impl Delete {
    pub fn parse(mut data: DataChunkFrame) -> Result<Self> {
        let Ok(key) = data.next_as_str() else {
            return Err(Box::new(DeleteError::NoKey));
        };

        Ok(Self { key: Some(key) })
    }

    pub async fn respond(self, conn: Connection, db: DataStoreWrapper) -> Result<()> {
        let Some(key) = self.key.as_ref() else {
            return Err(Box::new(DeleteError::NoKey));
        };

        let mut data_store_guard = db.db.write().await;

        // TODO: once TTL is figured out, it needs to be accounted for
        if let Some(value) = data_store_guard.db.remove(key) {
            conn.write_chunk(super::DataType::SimpleString, Some("1".as_bytes()))
                .await?
        } else {
            conn.write_chunk(super::DataType::SimpleString, Some("0".as_bytes()))
                .await?
        }

        Ok(())
    }
}
