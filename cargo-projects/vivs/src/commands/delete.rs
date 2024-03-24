use super::CommonCommand;
use crate::{
    data_chunk::DataChunkFrame,
    utils::{u64_as_bytes, INCORRECT_ARGS_ERR},
    Connection, DataStore, GenericResult,
};
use log::info;

pub const DELETE_CMD: &str = "delete";

#[derive(Debug, Default)]
pub struct Delete {
    key: Option<String>,
}

impl CommonCommand for Delete {
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

        // TODO: once TTL is figured out, it needs to be accounted for
        if let Some(_value) = data_store_guard.remove(key) {
            let val = u64_as_bytes(1);
            conn.write_chunk(super::DataType::Integer, Some(&val))
                .await?
        } else {
            let val = u64_as_bytes(0);
            conn.write_chunk(super::DataType::Integer, Some(&val))
                .await?
        }

        info!(
            "{}",
            format!(
                "{:?} {:?} {:?}",
                conn.connected_peer_addr(),
                DELETE_CMD.to_uppercase(),
                self.key.as_ref().unwrap()
            )
        );

        Ok(())
    }
}
