use crate::DataStoreWrapper;
use crate::Listener;
use tokio::net::TcpListener;

// New improved way of handling requests
pub async fn start(addr: String, port: String) -> crate::Result<()> {
    let address = format!("{}:{}", addr, port);
    let tcp_listener = TcpListener::bind(&address).await?;

    let listener = Listener {
        tcp_listener,
        db: DataStoreWrapper::new(),
    };

    // this should return a frame / bits of data
    // that will get parsed into commands
    listener.run().await?;

    // TODO - command to do the work

    Ok(())
}
