use crate::{Config, DataStore, GenericResult, Listener, NodeListener, VIVS_CONFIG_LAZY};
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
    let Config {
        connection,
        cluster,
    } = &vivs_config.unwrap();

    let args = Cli::parse();
    let port = args.port.unwrap_or(connection.port);
    let address = &connection.address;
    let cluster = cluster.as_ref();

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

    // Cluster mode enabled
    if let Some(cluster) = cluster.as_ref() {
        if cluster.enabled {
            let cluster_port = if let Some(cluster_port) = cluster.port {
                cluster_port
            } else {
                port + 5000
            };

            info!("Attempting to bind on port {cluster_port}");

            // This is for node to node / peer to peer connections
            let node_tcp_listener = TcpListener::bind(format!("{address}:{cluster_port}"))
                .await
                .map_err(|err| {
                    error!("Failed to bind: {err}");
                    err
                })?;
            let node_listener = NodeListener::new(node_tcp_listener);

            let _ = tokio::join!(listener.run(), node_listener.run());

            return Ok(());
        }
    }

    // Enables to wait on concurrent branches, returning when all branches complete
    let _ = listener.run().await;

    Ok(())
}
