use super::CommonCommand;
use crate::{
    data_chunk::DataChunkFrame,
    utils::{INCORRECT_ARGS_ERR, VALUE_NOT_INT_ERR},
    Connection, DataStore, GenericResult,
};
use log::info;
use std::time::{Duration, SystemTime};

pub const SET_CMD: &str = "set";

#[derive(Default)]
pub struct Set {
    key: Option<String>,
    value: Option<String>,
    expiration: Option<u64>,
}

impl CommonCommand for Set {
    fn parse(mut data: DataChunkFrame) -> Self {
        // we try to get the key first
        let Ok(key) = data.next_as_str() else {
            return Self::default();
        };
        // then the value
        let Ok(value) = data.next_as_str() else {
            return Self::default();
        };
        // and then expiration
        let expiration = if let Ok(Some(option)) = data.next_as_str() {
            // check option
            // *"expire" dereferences the static reference which is a string allocated in the read-only memory
            if option.to_lowercase() == *"expire" {
                if let Ok(Some(ex_val)) = data.next_as_str() {
                    let val = ex_val.parse::<u32>();
                    if let Ok(v) = val {
                        let current_system_time = SystemTime::now();

                        let expiry_s_u64 = u64::from(v);

                        let expiry_duration_s = Duration::from_secs(expiry_s_u64);
                        let expiry_time = current_system_time + expiry_duration_s;

                        let ttl = expiry_time
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

                        Some(ttl)
                    } else {
                        Some(0)
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        Self {
            key,
            value,
            expiration,
        }
    }

    async fn respond(&self, connection: &mut Connection, db: &DataStore) -> GenericResult<()> {
        // key missing
        let Some(key) = self.key.as_ref() else {
            connection
                .write_error(INCORRECT_ARGS_ERR.as_bytes())
                .await?;
            return Ok(());
        };

        // value missing
        let Some(value) = self.value.as_ref() else {
            connection
                .write_error(INCORRECT_ARGS_ERR.as_bytes())
                .await?;
            return Ok(());
        };

        let mut data_store_guard = db.db.write().await;
        data_store_guard.insert(key.to_owned(), value.to_owned());

        if let Some(expiration) = self.expiration {
            if expiration == 0 {
                connection.write_error(VALUE_NOT_INT_ERR.as_bytes()).await?;
                return Ok(());
            }

            let mut expirations_data_store_guard = db.expirations.write().await;
            expirations_data_store_guard.insert(key.to_owned(), expiration);
        };

        info!(
            "{}",
            format!(
                "{:?} {:?} {:?}",
                connection.connected_peer_addr(),
                SET_CMD.to_uppercase(),
                self.key
            )
        );

        connection
            .write_chunk(super::DataType::SimpleString, Some("OK".as_bytes()))
            .await?;

        Ok(())
    }
}
