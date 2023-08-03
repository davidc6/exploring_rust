use std::fmt::Display;
use bytes::Buf;
use crate::{
    data_chunk::{DataChunk, DataChunkFrame},
    Connection, DataStoreWrapper, Error, Result,
};

pub struct Set {
    key: Option<DataChunk>,
    value: Option<DataChunk>,
}

#[derive(Debug)]
pub enum CommandError {
    NonParsableCommand,
    UnknownCommand,
}

impl std::error::Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Set {
    pub fn parse(mut data: DataChunkFrame) -> Result<Self> {
        // TODO: peek instead of next so that we can know if actual value exists

        // TODO: a better way to parse values
        // let length = data.len;
        // if length < 2 {
        //     return Err(Box::new(CommandError::NonParsableCommand));
        // }
        // let mut key = None;
        // let mut value = None;
        // for (position, data_chunk) in data.enumerate() {
        //     match position {
        //         0 => key = Some(data_chunk),
        //         _ => value = Some(data_chunk),
        //     }
        // }
        // Ok(Self { key, value })

        match data.next() {
            Ok(key) => match data.next() {
                Ok(value) => Ok(Self {
                    key: Some(key),
                    value: Some(value),
                }),
                Err(e) => Err(e.into()),
            },
            Err(e) => Err(e.into()),
        }
    }

    pub async fn respond(&self, connection: Connection, db: DataStoreWrapper) -> Result<()> {
        let value_result = match self.value.as_ref().unwrap() {
            DataChunk::Bulk(value) => {
                let val = std::str::from_utf8(value.chunk());

                val
            }
            _ => {
                // TODO
                panic!("Error");
            }
        };

        match self.key.as_ref().unwrap() {
            DataChunk::Bulk(key) => {
                let key = std::str::from_utf8(key.chunk());

                if key.is_err() {
                    return Err("Failed to convert to UTF8".into());
                }

                let key = key.unwrap();

                let mut data_store_guard = db.db.write().await;

                if data_store_guard
                    .db
                    .insert(key.to_owned(), value_result.unwrap().to_owned())
                    .is_none()
                {
                    connection
                        .write_chunk(
                            super::DataType::SimpleString,
                            Some(value_result.unwrap().to_owned().as_bytes()),
                        )
                        .await?
                } else {
                    connection.write_chunk(super::DataType::Null, None).await?
                }
            }
            _ => {
                // TODO
                panic!("Error");
            }
        }
        Ok(())
    }
}
