use log::info;

use crate::data_chunk::DataChunkFrame;
use crate::utils::format_err_msg;
use crate::{Connection, DataStoreWrapper, Result};

const GET_CMD: &str = "GET";

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

    pub async fn respond(&self, conn: &mut Connection, db: &DataStoreWrapper) -> Result<()> {
        let Some(key) = self.key.as_ref() else {
            // TODO: extract error type into a separate function
            let e = format_err_msg("Incorrect number of arguments".to_owned());
            conn.write_error(e.as_bytes()).await?;
            return Ok(());
        };

        let data_store_guard = db.db.read().await;

        info!(
            "{}",
            format!(
                "{:?} {:?} {:?}",
                conn.connected_peer_addr(),
                GET_CMD,
                self.key.as_ref().unwrap()
            )
        );

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
