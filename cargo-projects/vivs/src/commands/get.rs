use crate::data_chunk::DataChunkFrame;
use crate::utils::num_args_err;
use crate::{Connection, DataStoreWrapper, Result};
use log::info;

const GET_CMD: &str = "GET";

pub struct Get {
    key: Option<String>,
}

impl Get {
    pub fn parse(mut data: DataChunkFrame) -> Self {
        let Ok(key) = data.next_as_str() else {
            // Setting key to None here will write error message to tcp stream
            return Self { key: None };
        };

        Self { key }
    }

    pub async fn respond(&self, conn: &mut Connection, db: &DataStoreWrapper) -> Result<()> {
        let Some(key) = self.key.as_ref() else {
            conn.write_error(num_args_err().as_bytes()).await?;
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
        // i.e. if expired expire and do not return
        if let Some(value) = data_store_guard.db.get(key) {
            conn.write_chunk(super::DataType::SimpleString, Some(value.as_bytes()))
                .await?
        } else {
            conn.write_null().await?
        }

        Ok(())
    }
}
