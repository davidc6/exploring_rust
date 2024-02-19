use crate::{DataStore, GenericResult, Listener};
use log::{error, info};
use tokio::net::TcpListener;

pub async fn start(ipv4: String, port: String) -> GenericResult<()> {
    let address = format!("{}:{}", ipv4, port);
    info!("Attempting to bind on port {port}");

    // bind/assign address to the socket (ip address + port number)
    let tcp_listener = TcpListener::bind(&address).await.map_err(|err| {
        error!("TCP listener failed to bind: {err}");
        err
    })?;

    let listener = Listener::new(tcp_listener, DataStore::new());
    listener.run().await?;

    Ok(())
}
