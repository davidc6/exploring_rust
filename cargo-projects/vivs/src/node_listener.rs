use crate::{Connection, DataStore, GenericResult, Handler};
use log::{error, info};
use tokio::net::TcpListener;

pub struct NodeListener {
    tcp_listener: TcpListener,
}

impl NodeListener {
    /// Creates a `Listener`.
    ///
    /// `TcpListener` and `DataStore` get injected via the two parameters.
    pub fn new(tcp_listener: TcpListener) -> Self {
        NodeListener { tcp_listener }
    }

    /// Starts listening to the incoming connections and processes accordingly.
    pub async fn run(self) -> GenericResult<()> {
        info!(
            "Listening for connections on {}",
            self.tcp_listener.local_addr()?
        );

        // To accept multiple incoming connections,
        // a loop construct is used here to handle each connection.
        // It is handled as a separate task (either on the current or different thread).
        // Then a loop inside each thread is used to handle incoming data from the client socket.
        loop {
            // waits to accept a new connection from the tcp listener
            let (tcp_stream, socket_addr) = self.tcp_listener.accept().await?;

            info!("Incoming connection request from {:?}", socket_addr);

            // let mut handler = Handler::new(self.db.clone(), Connection::new(tcp_stream));

            // Creates a new task.
            // A Tokio task is an async green (aka virtual) thread that is created by a runtime of VM (instead of OS).
            // Tasks are created by passing an async block to spawn().
            tokio::spawn(async move {
                info!("Connection established with {:?}", socket_addr);
                // TODO
            });
        }
    }
}
