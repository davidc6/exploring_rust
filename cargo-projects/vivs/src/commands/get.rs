use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::CommonCommand;
use crate::data_chunk::DataChunkFrame;
use crate::utils::INCORRECT_ARGS_ERR;
use crate::{Connection, DataStore, GenericResult};
use log::info;

pub const GET_CMD: &str = "get";

pub struct Get {
    key: Option<String>,
}

impl CommonCommand for Get {
    fn parse(mut data: DataChunkFrame) -> Self {
        let Ok(key) = data.next_as_str() else {
            return Self { key: None };
        };

        Self { key }
    }

    async fn respond(&self, conn: &mut Connection, db: &DataStore) -> GenericResult<()> {
        let Some(key) = self.key.as_ref() else {
            conn.write_error(INCORRECT_ARGS_ERR.as_bytes()).await?;
            return Ok(());
        };

        let mut data_store_guard = db.db.write().await;

        info!(
            "{}",
            format!(
                "{:?} {:?} {:?}",
                conn.connected_peer_addr(),
                GET_CMD.to_uppercase(),
                self.key.as_ref().unwrap()
            )
        );

        // TODO: once TTL is figured out, it needs to be accounted for
        // i.e. if expired expire and do not return
        // let v = data_store_guard.get(key);
        // drop(data_store_guard);

        if let Some(v) = data_store_guard.get(key) {
            // check for expiration
            let mut expires = db.expirations.write().await;

            if let Some(ttl) = expires.get(key) {
                let current = SystemTime::now();
                let current_s = current.duration_since(UNIX_EPOCH).unwrap();

                if Duration::from_secs(*ttl) <= current_s {
                    data_store_guard.remove(key);
                    expires.remove(key);

                    conn.write_null().await?
                } else {
                    conn.write_chunk(super::DataType::SimpleString, Some(v.as_bytes()))
                        .await?
                }
            } else {
                conn.write_chunk(super::DataType::SimpleString, Some(v.as_bytes()))
                    .await?
            }
        } else {
            conn.write_null().await?
        }

        Ok(())
    }
}
