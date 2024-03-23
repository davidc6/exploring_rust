use super::CommonCommand;
use crate::data_chunk::DataChunkFrame;
use crate::utils::INCORRECT_ARGS_ERR;
use crate::{Connection, DataStore, GenericResult};
use log::info;
use std::time::{Duration, SystemTime};

pub const TTL_CMD: &str = "ttl";

pub struct Ttl {
    key: Option<String>,
}

impl CommonCommand for Ttl {
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

        let data_store_guard = db.expirations.read().await;

        info!(
            "{}",
            format!(
                "{:?} {:?} {:?}",
                conn.connected_peer_addr(),
                TTL_CMD.to_uppercase(),
                self.key.as_ref().unwrap()
            )
        );

        // TODO: once TTL is figured out, it needs to be accounted for
        // i.e. if expired expire and do not return
        if let Some(expiry_s) = data_store_guard.get(key) {
            let current_time = SystemTime::now();

            let expiry_duration_s = Duration::from_secs(*expiry_s);
            let current_duration_s = current_time.duration_since(SystemTime::UNIX_EPOCH)?;

            let ttl = if expiry_duration_s <= current_duration_s {
                Duration::from_secs(0)
            } else {
                expiry_duration_s - current_duration_s
            }
            .as_secs();

            let ttl_byte_arr = if cfg!(target_endian = "big") {
                ttl.to_be_bytes()
            } else {
                ttl.to_le_bytes()
            };

            conn.write_chunk(super::DataType::Integer, Some(&ttl_byte_arr))
                .await?
        } else {
            conn.write_null().await?
        }

        Ok(())
    }
}
