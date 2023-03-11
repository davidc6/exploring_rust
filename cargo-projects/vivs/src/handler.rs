use crate::{Command, Connection, DataStoreWrapper, Result};

// pub mod handler {
//     trait Handling {
//         fn run() -> Result<(), ()>;
//     }

pub struct Handler {
    pub db: DataStoreWrapper,
    pub tcp_connection: Connection, // pub stream: TcpStream,
}

impl Handler {
    pub async fn run(self) -> Result<()> {
        println!("Hello");

        let cmd = Command::parse_cmd().unwrap();
        // pass db and connection
        cmd.run(self.tcp_connection).await?;

        // self.tcp_connection.write_chunk(data)

        Ok(())
    }
}
// }
