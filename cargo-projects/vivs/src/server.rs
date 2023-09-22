use crate::DataStoreWrapper;
use crate::Listener;
use tokio::net::TcpListener;

// New improved way of handling requests
pub async fn start(addr: String, port: String) -> crate::Result<()> {
    let address = format!("{}:{}", addr, port);
    // bind/asign address to the socket (ip address + port number)
    let tcp_listener = TcpListener::bind(&address).await?;

    // listener construct, listens to incoming connections and assembles their processing
    let listener = Listener {
        tcp_listener,
        db: DataStoreWrapper::new(),
    };

    // this should return a frame / bits of data
    // that will get parsed into commands
    listener.run().await?;

    Ok(())
}
