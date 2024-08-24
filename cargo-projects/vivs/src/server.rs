use crate::{DataStore, GenericResult, Listener, VIVS_CONFIG};
use log::{error, info};
use tokio::net::TcpListener;

pub async fn start() -> GenericResult<()> {
    let vivs_config_ref = VIVS_CONFIG.with(|arc| arc.clone());
    let vivs_config = vivs_config_ref.lock();

    let v = vivs_config.await;
    let port = v.get("port").unwrap();

    let address = format!("{}:{}", v.get(&"address".to_owned()).unwrap(), port);
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
