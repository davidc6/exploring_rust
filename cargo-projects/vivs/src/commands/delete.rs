use super::CommonCommand;
use crate::{
    commands::ARGS_NUM, parser::Parser, utils::u64_as_bytes, Connection, DataStore, GenericResult,
};
use log::info;

pub const DELETE_CMD: &str = "delete";

#[derive(Debug, Default)]
pub struct Delete {
    key: Option<String>,
}

impl CommonCommand for Delete {
    fn parse(mut data: Parser) -> Self {
        let Ok(key) = data.next_as_str() else {
            return Self { key: None };
        };

        Self { key }
    }

    async fn respond(&self, conn: &mut Connection, db: &DataStore) -> GenericResult<()> {
        let Some(key) = self.key.as_ref() else {
            conn.write_error(ARGS_NUM.as_bytes()).await?;
            return Ok(());
        };

        let mut db_guard = db.db.write().await;

        if let Some(_value) = db_guard.remove(key) {
            let mut expiries_guard = db.expirations.write().await;
            expiries_guard.remove(key);

            let total_entries_deleted = u64_as_bytes(1);
            conn.write_chunk(super::DataType::Integer, &total_entries_deleted)
                .await?
        } else {
            let total_entries_deleted = u64_as_bytes(0);
            conn.write_chunk(super::DataType::Integer, &total_entries_deleted)
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
