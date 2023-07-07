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
        let result = db.db.read().unwrap();
        let b = result.db.get("helo");

        println!("RESPONSE {:?}", b);

        Ok(())
    }
}
