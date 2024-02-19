use super::CommonCommand;
use crate::{
    data_chunk::DataChunkFrame, utils::INCORRECT_ARGS_ERR, Connection, DataStore, GenericResult,
};
use log::info;

pub const SET_CMD: &str = "set";

#[derive(Default)]
pub struct Set {
    key: Option<String>,
    value: Option<String>,
}

impl CommonCommand for Set {
    fn parse(mut data: DataChunkFrame) -> Self {
        // we try to get the key first
        let Ok(key) = data.next_as_str() else {
            return Self::default();
        };
        // and then the value
        let Ok(value) = data.next_as_str() else {
            return Self::default();
        };

        Self { key, value }
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

        let mut data_store_guard = db.db.write().await;

        info!(
            "{}",
            format!(
                "{:?} {:?} {:?}",
                connection.connected_peer_addr(),
                SET_CMD.to_uppercase(),
                self.key.as_ref().unwrap()
            )
        );

        data_store_guard.insert(key.to_owned(), value.to_owned());
        connection
            .write_chunk(super::DataType::SimpleString, Some("OK".as_bytes()))
            .await?;

        Ok(())
    }
}
