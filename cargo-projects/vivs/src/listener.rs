use crate::{Connection, DataStoreWrapper, Handler, Result};
use log::{error, info};
use tokio::net::TcpListener;

pub struct Listener {
    pub tcp_listener: TcpListener,
    pub db: DataStoreWrapper,
}

impl Listener {
    pub fn new(tcp_listener: TcpListener, db: DataStoreWrapper) -> Self {
        Listener { tcp_listener, db }
    }

    pub async fn run(self) -> Result<()> {
        info!("Server initialised");
        info!("Listening for connections");

        // To accept multiple incoming connections,
        // loop construct is used here to handle each connection.
        // as a separate task (either on the current or different thread)
        // Then a loop inside each thread is used to handle incoming data from client socket
        loop {
            // wait to accept a new connection from the tcp listener
            let (tcp_stream, socket_addr) = self.tcp_listener.accept().await?;

            info!("Incoming connection request from {:?}", socket_addr);

            let mut handler = Handler {
                // As the db is wrapped in an Arc, we use .clone() here to produce a new instance
                // which points to the same allocation as source and increases the reference count
                db: self.db.clone(),
                // Connection instance - buffer allocation and frame (network data) parsing occurs here
                connection: Connection::new(tcp_stream),
            };

            // Create a new task.
            // A Tokio task is an async green (aka virtual) thread that is created by a runtime of VM (instead of OS).
            // Tasks are created by passing an async block to spawn().
            tokio::spawn(async move {
                info!("Connection established with {:?}", socket_addr);
                // Wait for me data from already connected sockets,
                // by looping here the connection does not close.
                // If we don't loop and when a client tries to send data continuously on the socket,
                // we'll get the "brokne pipe" error message.
                loop {
                    match handler.run().await {
                        Ok(_) => (),
                        Err(e) => {
                            error!("Failed to handle the request: {:?}", e);
                            break;
                        }
                    };
                }
            });
        }
    }
}
