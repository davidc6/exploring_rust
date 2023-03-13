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
        println!("Listening ...");

        loop {
            let (stream, addr) = self.listener.accept().await?;

            println!("Incoming from {:?}", addr);

            let connection = Connection::new(stream);

            let handler = Handler {
                db: self.db.clone(),
                tcp_connection: connection,
            };

            tokio::spawn(async move { handler.run().await });
        }
    }
}
