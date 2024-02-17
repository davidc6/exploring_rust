use crate::{
    data_chunk::DataChunkFrame, utils::num_args_err, Connection, DataStoreWrapper, Result,
};

#[derive(Default)]
pub struct Set {
    key: Option<String>,
    value: Option<String>,
}

impl Set {
    pub fn parse(mut data: DataChunkFrame) -> Self {
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

    pub async fn respond(&self, connection: &mut Connection, db: &DataStoreWrapper) -> Result<()> {
        let Some(key) = self.key.as_ref() else {
            connection.write_error(num_args_err().as_bytes()).await?;
            return Ok(());
        };

        let Some(value) = self.value.as_ref() else {
            connection.write_error(num_args_err().as_bytes()).await?;
            return Ok(());
        };

        let mut data_store_guard = db.db.write().await;

        data_store_guard.db.insert(key.to_owned(), value.to_owned());
        connection
            .write_chunk(super::DataType::SimpleString, Some("OK".as_bytes()))
            .await?;

        Ok(())
    }
}
