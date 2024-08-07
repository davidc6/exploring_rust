use super::CommonCommand;
use crate::parser::Parser;
use crate::utils::INCORRECT_ARGS_ERR;
use crate::{Connection, DataStore, GenericResult};
use log::info;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const GET_CMD: &str = "get";

pub struct Get {
    key: Option<String>,
}

impl CommonCommand for Get {
    fn parse(mut data: Parser) -> Self {
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

        let mut db_guard = db.db.write().await;

        info!(
            "{}",
            format!(
                "{:?} {:?} {:?}",
                conn.connected_peer_addr(),
                GET_CMD.to_uppercase(),
                self.key.as_ref().unwrap()
            )
        );

        if let Some(value) = db_guard.get(key) {
            let mut expiries_guard = db.expirations.write().await;

            // If key exists in cache then check for TTL and whether the value has expired.
            // If key exists but expired (i.e. current time is more than expiry time),
            // evict from both stores and return null.
            if let Some(unix_time) = expiries_guard.get(key) {
                let duration_now_s = SystemTime::now().duration_since(UNIX_EPOCH)?;

                if Duration::from_secs(*unix_time) <= duration_now_s {
                    db_guard.remove(key);
                    expiries_guard.remove(key);

                    conn.write_null().await?
                } else {
                    conn.write_chunk(super::DataType::SimpleString, Some(value.as_bytes()))
                        .await?
                }
            } else {
                conn.write_chunk(super::DataType::SimpleString, Some(value.as_bytes()))
                    .await?
            }
        } else {
            conn.write_null().await?
        }

        Ok(())
    }
}
