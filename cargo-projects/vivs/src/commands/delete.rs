use crate::{
    data_chunk::DataChunkFrame, utils::INCORRECT_ARGS_ERR, Connection, DataStore, GenericResult,
};

#[derive(Debug, Default)]
pub struct Delete {
    key: Option<String>,
}

impl Delete {
    pub fn parse(mut data: DataChunkFrame) -> Self {
        let Ok(key) = data.next_as_str() else {
            return Self { key: None };
        };

        Self { key }
    }

    pub async fn respond(self, conn: &mut Connection, db: &DataStore) -> GenericResult<()> {
        let Some(key) = self.key.as_ref() else {
            conn.write_error(INCORRECT_ARGS_ERR.as_bytes()).await?;
            return Ok(());
        };

        let mut data_store_guard = db.db.write().await;

        // TODO: once TTL is figured out, it needs to be accounted for
        if let Some(_value) = data_store_guard.remove(key) {
            conn.write_chunk(super::DataType::Integer, Some("1".as_bytes()))
                .await?
        } else {
            conn.write_chunk(super::DataType::Integer, Some("0".as_bytes()))
                .await?
        }

        Ok(())
    }
}
