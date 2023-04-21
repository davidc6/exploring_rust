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

        // To accept multiple incoming connections,
        // loop construct is used here to handle each connection
        // as a separate task (either on the current or different thread)
        loop {
            // wait for the new connection to establish
            let (tcp_stream, socket_addr) = self.tcp_listener.accept().await?;
            println!("Incoming request from {:?}", socket_addr);

            // A connection handler per connection
            let handler = Handler {
                // produces new instance which points to the same allocation as source and increases the reference count
                db: self.db.clone(),
                // connection instance - buffer allocation and frame parsing occurs here
                connection: Connection::new(tcp_stream),
            };

            // spawn a new task which might end up executing on the same or different thread,
            // depending on the Tokio scheduler
            tokio::spawn(async move { handler.run().await });
        }
    }
}
