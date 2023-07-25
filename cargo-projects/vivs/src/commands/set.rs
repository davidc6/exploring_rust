use crate::{
    data_chunk::{DataChunk, DataChunkFrame},
    Connection, DataStoreWrapper, Result,
};

pub struct Set {
    key: Option<DataChunk>,
    value: Option<DataChunk>,
}

impl Set {
    pub fn parse(mut data: DataChunkFrame) -> Result<Self> {
        // TODO: peek instead of next so that we can know if actual value exists
        match data.next() {
            Ok(data) => Ok(Set {
                key: Some(data),
                value: None,
            }),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn respond(&self, connection: Connection, db: DataStoreWrapper) -> Result<()> {
        println!("SET");
        Ok(())
    }
}
