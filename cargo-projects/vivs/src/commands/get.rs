use crate::data_chunk::DataChunkFrame;
use crate::Result;
use crate::{Connection, DataStoreWrapper};
use std::fmt::Display;

#[derive(Debug)]
enum GetError {
    NoKey,
    IncorrectNumberOfArgs,
}

impl std::error::Error for GetError {}

impl Display for GetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GetError::NoKey => write!(f, "No key was passed to GET command"),
            GetError::IncorrectNumberOfArgs => write!(f, "Not enough arguments"),
        }
    }
}

pub struct Get {
    key: Option<String>,
}

impl Get {
    pub fn parse(mut data: DataChunkFrame) -> Result<Self> {
        let Ok(key) = data.next_as_str() else {
            // Setting key to None will force error message to be written back to tcp stream
            return Ok(Self { key: None });
        };

        Ok(Self { key: Some(key) })
    }

    pub async fn respond(self, conn: &mut Connection, db: &DataStoreWrapper) -> Result<()> {
        let Some(key) = self.key.as_ref() else {
            conn.write_error("ERR Incorrect number of arguments\r\n".as_bytes())
                .await?;
            return Ok(());
        };

        let data_store_guard = db.db.read().await;

        // TODO: once TTL is figured out, it needs to be accounted for
        if let Some(value) = data_store_guard.db.get(key) {
            conn.write_chunk(super::DataType::SimpleString, Some(value.as_bytes()))
                .await?
        } else {
            conn.write_null().await?
        }

        Ok(())
    }
}
