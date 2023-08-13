use crate::{data_chunk::DataChunkFrame, Connection, Result};

#[derive(Debug, Default)]
pub struct Ping {
    message: Option<String>,
}

impl Ping {
    pub fn new(message: Option<String>) -> Self {
        Ping { message }
    }

    pub fn parse(mut data: DataChunkFrame) -> Result<Self> {
        match data.next_as_str() {
            Ok(value) => Ok(Ping::new(Some(value))),
            Err(_) => Ok(Ping::default()),
        }
    }

    pub async fn respond(self, conn: Connection) -> Result<()> {
        if let Some(message) = self.message {
            conn.write_chunk(super::DataType::SimpleString, Some(message.as_bytes()))
                .await?
        } else {
            let default_response = "PONG";
            conn.write_chunk(
                super::DataType::SimpleString,
                Some(default_response.as_bytes()),
            )
            .await?
        }
        Ok(())
    }
}
