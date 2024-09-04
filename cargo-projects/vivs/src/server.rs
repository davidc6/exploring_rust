use crate::{DataStore, GenericResult, Listener, VIVS_CONFIG_LAZY};
use log::{error, info};
use tokio::net::TcpListener;

pub async fn start() -> GenericResult<()> {
    let vivs_config = &*VIVS_CONFIG_LAZY;
    let vivs_config = vivs_config.as_ref();

    let port;
    let address;

    if let Ok(vivs_config) = vivs_config.as_ref() {
        info!("Config found!");
        port = vivs_config.connection.port;
        address = format!("{}:{}", vivs_config.connection.address, port);
    } else {
        info!("No config found, using defaults");
        port = 9000;
        address = format!("{}:{}", "127.0.0.1", port);
    }

    info!("Attempting to bind on port {port} {address}");

    // Bind/assign the address to the socket (ip address + port number)
    let tcp_listener = TcpListener::bind(address).await.map_err(|err| {
        error!("TCP listener failed to bind: {err}");
        err
    })?;

    let listener = Listener::new(tcp_listener, DataStore::new());
    listener.run().await?;

    Ok(())
}
