use super::CommonCommand;
use crate::{
    data_chunk::DataChunkFrame,
    utils::{INCORRECT_ARGS_ERR, VALUE_NOT_INT_ERR},
    Connection, DataStore, GenericResult,
};
use log::info;
use std::{
    borrow::Cow,
    sync::Arc,
    time::{Duration, SystemTime},
};

pub const SET_CMD: &str = "set";

// xs - (x)expire in (s)seconds (e.g. SET greeting hello XS 60)
// xm - (x)expire in (m)minutes (TODO: not implemented)
const EXPIRE_SECONDS: &str = "xs";

#[derive(Default)]
pub struct Set {
    key: Option<String>,
    value: Option<String>,
    expiry: Option<u64>,
}

impl CommonCommand for Set {
    fn parse(mut data: DataChunkFrame) -> Self {
        // Get the key first
        let Ok(key) = data.next_as_str() else {
            return Self::default();
        };
        // Get the value second
        let Ok(value) = data.next_as_str() else {
            return Self::default();
        };
        // Get the expiry value last
        if let Ok(Some(option)) = data.next_as_str() {
            // Check for options (e.g. expire)
            // *"expire" dereferences the static reference which is a string allocated in the read-only memory
            if option.to_lowercase() == *EXPIRE_SECONDS {
                // Expire value (seconds)
                if let Ok(Some(expiry_val_as_string)) = data.next_as_str() {
                    // Parse to u64
                    if let Ok(expiry_s_u64) = expiry_val_as_string.parse::<u64>() {
                        let current_system_time = SystemTime::now();
                        let expiry_time = current_system_time + Duration::from_secs(expiry_s_u64);

                        let ttl = expiry_time
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

                        return Self {
                            key,
                            value,
                            expiry: Some(ttl),
                        };
                    }
                }
            }
        }

        Self {
            key,
            value,
            expiry: None,
        }
    }

    async fn respond(&self, connection: &mut Connection, db: &DataStore) -> GenericResult<()> {
        let Some(key) = self.key.as_ref() else {
            connection
                .write_error(INCORRECT_ARGS_ERR.as_bytes())
                .await?;
            return Ok(());
        };

        let Some(value) = self.value.as_ref() else {
            connection
                .write_error(INCORRECT_ARGS_ERR.as_bytes())
                .await?;
            return Ok(());
        };

        let mut db_guard = db.db.write().await;

        db_guard.insert(key.clone(), value.to_owned());

        if let Some(expiration) = self.expiry {
            if expiration == 0 {
                connection.write_error(VALUE_NOT_INT_ERR.as_bytes()).await?;
                return Ok(());
            }

            let mut expirations_data_store_guard = db.expirations.write().await;
            expirations_data_store_guard.insert(key.clone(), expiration);
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
