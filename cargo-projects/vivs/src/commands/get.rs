use crate::data_chunk::DataChunkFrame;
use crate::Result;
use crate::{Connection, DataStoreWrapper};

pub struct Get {}

impl Get {
    pub fn parse(mut data: DataChunkFrame) -> Result<Self> {
        // TODO: peek instead of next so that we can
        match data.next() {
            Ok(data) => Ok(Get {}),
            Err(e) => Err(e.into()), // Err(e) => Err(e.into()),
        }
    }

    pub async fn respond(self, conn: Connection, db: DataStoreWrapper) -> Result<()> {
        let data_store_guard = db.db.read().await;

        if let Some(value) = data_store_guard.db.get("helo") {
            conn.write_chunk(super::DataType::Null, Some(value.as_bytes()))
                .await?;
            Ok(())
        } else {
            let null = b"_";
            conn.write_chunk(super::DataType::Null, None).await?;
            Ok(())
        }

        //   Ok(())
    }
}
