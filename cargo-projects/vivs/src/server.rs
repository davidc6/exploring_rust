use crate::{DataStore, GenericResult, Listener, VIVS_CONFIG};
use log::{error, info};
use tokio::net::TcpListener;

pub async fn start() -> GenericResult<()> {
    let vivs_config = VIVS_CONFIG.get().unwrap();
    let port = vivs_config.connection.port;
    let address = format!("{}:{}", vivs_config.connection.address, port);
    info!("Attempting to bind on port {port}");

    // Bind/assign the address to the socket (ip address + port number)
    let tcp_listener = TcpListener::bind(address).await.map_err(|err| {
        error!("TCP listener failed to bind: {err}");
        err
    })?;

    let listener = Listener::new(tcp_listener, DataStore::new());
    listener.run().await?;

    Ok(())
}
