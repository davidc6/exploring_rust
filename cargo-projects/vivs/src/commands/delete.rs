use crate::{
    data_chunk::DataChunkFrame, utils::num_args_err, Connection, DataStoreWrapper, Result,
};

#[derive(Debug, Default)]
pub struct Delete {
    key: Option<String>,
}

impl Delete {
    pub fn parse(mut data: DataChunkFrame) -> Result<Self> {
        let Ok(key) = data.next_as_str() else {
            return Ok(Self { key: None });
        };

        Ok(Self { key: Some(key) })
    }

    pub async fn respond(self, conn: &mut Connection, db: &DataStoreWrapper) -> Result<()> {
        let Some(key) = self.key.as_ref() else {
            conn.write_error(num_args_err().as_bytes()).await?;
            return Ok(());
        };

        let mut data_store_guard = db.db.write().await;

        // TODO: once TTL is figured out, it needs to be accounted for
        if let Some(_value) = data_store_guard.db.remove(key) {
            conn.write_chunk(super::DataType::Integer, Some("1".as_bytes()))
                .await?
        } else {
            conn.write_chunk(super::DataType::Integer, Some("0".as_bytes()))
                .await?
        }

        Ok(())
    }
}
