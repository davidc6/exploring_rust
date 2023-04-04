use crate::{Connection, DataStoreWrapper, Handler, Result};
use tokio::net::TcpListener;

pub struct Listener {
    pub tcp_listener: TcpListener,
    pub db: DataStoreWrapper, // should be clonable
}

impl Listener {
    pub fn new(tcp_listener: TcpListener, db: DataStoreWrapper) -> Self {
        Listener { tcp_listener, db }
    }

    pub async fn run(self) -> Result<()> {
        println!("Listening for requests...");

        loop {
            let (tcp_stream, socket_addr) = self.tcp_listener.accept().await?;
            println!("Incoming request from {:?}", socket_addr);

            // create a connnection instance
            let mut connection = Connection::new(tcp_stream);
            // read bits that host/client can send (frame)
            let number = connection.read_and_process_stream().await?;

            let handler = Handler {
                db: self.db.clone(), // produces new instance which points to the same allocation as source and increases the reference count
                connection,
            };

            // spawn a new task which might end up executing on the same or different thread,
            // depending on the Tokio scheduler
            tokio::spawn(async move { handler.run().await });
        }
    }
}
