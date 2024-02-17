use crate::DataStoreWrapper;
use crate::Listener;
use crate::Result;
use log::{error, info};
use tokio::net::TcpListener;

pub async fn start(ipv4: String, port: String) -> Result<()> {
    let address = format!("{}:{}", ipv4, port);
    info!("Attempting to bind on port {}", port);

    // bind/assign address to the socket (ip address + port number)
    let tcp_listener = TcpListener::bind(&address).await.map_err(|err| {
        error!("TCP listener failed to bind: {}", err);
        err
    })?;

    let listener = Listener {
        tcp_listener,
        db: DataStoreWrapper::new(),
    };

    // this should return a frame / bits of data
    // that will get parsed into commands
    listener.run().await?;

    Ok(())
}
