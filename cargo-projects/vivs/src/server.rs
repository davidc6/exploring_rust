use crate::{DataStore, GenericResult, Listener, Listener2, VIVS_CONFIG_LAZY};
use clap::Parser;
use log::{error, info};
use tokio::net::{tcp, TcpListener};

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

    info!("Attempting to bind on port {port}");

    // Bind/assign the address to the socket (ip address + port number)
    let tcp_listener = TcpListener::bind(format!("{address}:{port}"))
        .await
        .map_err(|err| {
            error!("TCP listener failed to bind: {err}");
            err
        })?;

    let tcp_listener = Listener::new(tcp_listener, DataStore::new());

    let tcp_listener_bus = TcpListener::bind(format!("{address}:10000"))
        .await
        .map_err(|err| {
            error!("TCP listener failed to bind: {err}");
            err
        })?;

    let tcp_listener_bus = Listener2::new(tcp_listener_bus);

    let _ = tokio::join!(tcp_listener.run(), tcp_listener_bus.run());

    Ok(())
}
