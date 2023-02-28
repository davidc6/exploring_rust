use crate::{Connection, DataStoreWrapper, Handler, Result};
use tokio::net::TcpListener;

pub struct Listener {
    pub listener: TcpListener,
    pub db: DataStoreWrapper, // should be clonable
}

impl Listener {
    pub fn new(listener: TcpListener, db: DataStoreWrapper) -> Self {
        Listener { listener, db }
    }

    pub async fn run(self) -> Result<()> {
        println!("Incoming connection");

        loop {
            // pass read and write instead
            let (stream, _) = self.listener.accept().await?;
            // let mut s = stream;
            // let (read, write) = s.split();

            let connection = Connection::new(stream);
            // let connection = Connection::new(read, write);

            let handler = Handler {
                db: self.db.clone().db,
                tcp_connection: connection,
            };

            tokio::spawn(async move { handler.run().await });
        }

        // Ok(())
    }
}
