use crate::{
    data_chunk::{DataChunk, DataChunkFrame},
    Connection, DataStoreWrapper, Error, Result,
};
use bytes::Buf;
use std::fmt::Display;

pub struct Set {
    key: Option<String>,
    value: Option<String>,
}

#[derive(Debug)]
pub enum CommandError {
    NonParsableCommand,
    UnknownCommand,
}

#[derive(Debug)]
pub enum SetError {
    NoKey,
    NoValue,
}

impl std::error::Error for SetError {}
impl std::error::Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Command error")
    }
}
impl Display for SetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Set error")
    }
}

impl Set {
    pub fn parse(mut data: DataChunkFrame) -> Result<Self> {
        let Ok(key) = data.next_as_str() else {
            return Err(Box::new(SetError::NoKey));
        };
        let Ok(value) = data.next_as_str() else {
            return Err(Box::new(SetError::NoValue));
        };
        Ok(Self {
            key: Some(key),
            value: Some(value),
        })
    }

    pub async fn respond(&self, connection: Connection, db: DataStoreWrapper) -> Result<()> {
        match self.key.clone() {
            Some(key) => {
                let mut data_store_guard = db.db.write().await;

                if data_store_guard
                    .db
                    .insert(key, self.value.clone().unwrap())
                    .is_none()
                {
                    connection
                        .write_chunk(
                            super::DataType::SimpleString,
                            Some(self.value.clone().unwrap().as_bytes()),
                        )
                        .await?
                } else {
                    connection.write_chunk(super::DataType::Null, None).await?
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}
