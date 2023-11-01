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
        // TODO: logging here
        println!(
            "Listening on {:?} for the incoming connections ...",
            self.tcp_listener.local_addr()?
        );

        // To accept multiple incoming connections,
        // loop construct is used here to handle each connection.
        // as a separate task (either on the current or different thread)
        loop {
            // wait to accept a new connection from the tcp listener
            let (tcp_stream, socket_addr) = self.tcp_listener.accept().await?;
            println!("Incoming request from {:?}", socket_addr);

            let mut handler = Handler {
                // As the db is wrapped in an Arc, we use .clone() here to produce a new instance
                // which points to the same allocation as source and increases the reference count
                db: self.db.clone(),
                // Connection instance - buffer allocation and frame (network data) parsing occurs here
                connection: Connection::new(tcp_stream),
            };

            // spawn a new task, by passing an async block to it a green thread is created
            tokio::spawn(async move {
                // Wait for me data from already connected sockets,
                // by looping here the connection does not close.
                // If we don't loop and when a client tries to send data continuously on the socket,
                // we'll get the "brokne pipe" error message.
                loop {
                    match handler.run().await {
                        Ok(_) => (),
                        Err(e) => {
                            // TODO: log error
                            println!("ERROR {:?}", e);
                            break;
                        }
                    };
                }
            });
        }
    }
}
