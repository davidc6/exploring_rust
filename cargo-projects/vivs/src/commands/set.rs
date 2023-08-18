use crate::{data_chunk::DataChunkFrame, Connection, DataStoreWrapper, Result};
use std::fmt::Display;

#[derive(Debug)]
pub enum CommandError {
    NonParsableCommand,
    UnknownCommand,
}
impl std::error::Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CommandError::NonParsableCommand => write!(f, "This command cannot be parsed"),
            CommandError::UnknownCommand => write!(f, "This command is unknown"),
        }
    }
}

#[derive(Debug)]
pub enum SetError {
    NoKey,
    NoValue,
}

impl std::error::Error for SetError {}

impl Display for SetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SetError::NoKey => write!(f, "No key was passed to SET command"),
            SetError::NoValue => write!(f, "No value was passed to SET command"),
        }
    }
}

pub struct Set {
    key: Option<String>,
    value: Option<String>,
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
        let Some(key) = self.key.as_ref() else {
            return Err(Box::new(SetError::NoKey));
        };

        let Some(value) = self.value.as_ref() else {
            return Err(Box::new(SetError::NoValue));
        };

        let mut data_store_guard = db.db.write().await;

        if data_store_guard
            .db
            .insert(key.to_owned(), value.to_owned())
            .is_none()
        {
            connection
                .write_chunk(super::DataType::SimpleString, Some(value.as_bytes()))
                .await?
        } else {
            connection.write_chunk(super::DataType::Null, None).await?
        }

        Ok(())
    }
}
