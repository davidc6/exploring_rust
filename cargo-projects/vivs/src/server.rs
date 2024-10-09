use crate::{DataStore, GenericResult, Listener, NodeListener, VIVS_CONFIG_LAZY};
use clap::Parser;
use log::{error, info};
use tokio::net::TcpListener;

#[derive(Parser)]
struct Cli {
    #[arg(long, short)]
    port: Option<u16>,
}

pub async fn start() -> GenericResult<()> {
    let vivs_config = &*VIVS_CONFIG_LAZY;
    let vivs_config = vivs_config.as_ref();
    let connection = &vivs_config.unwrap().connection;

    let args = Cli::parse();
    let port = args.port.unwrap_or(connection.port);
    let address = &connection.address;
    let cluster_port = &connection.cluster_port;

    info!("Vivs initialised");
    info!("Attempting to bind on port {port}");

    // Bind/assign the address to the socket (ip address + port number)
    // This is for client connections
    let tcp_listener = TcpListener::bind(format!("{address}:{port}"))
        .await
        .map_err(|err| {
            error!("Failed to bind: {err}");
            err
        })?;
    let listener = Listener::new(tcp_listener, DataStore::new());

    info!("Attempting to bind on port {cluster_port}");

    // This is for node to node / peer to peer connections
    let node_tcp_listener = TcpListener::bind(format!("{address}:{cluster_port}"))
        .await
        .map_err(|err| {
            error!("Failed to bind: {err}");
            err
        })?;
    let node_listener = NodeListener::new(node_tcp_listener);

    // Enables to wait on concurrent branches, returning when all branches complete
    let _ = tokio::join!(listener.run(), node_listener.run());

    Ok(())
}
