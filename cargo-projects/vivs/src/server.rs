use crate::{Config, ConnectionState, DataStore, GenericResult, Listener, VIVS_CONFIG_LAZY};
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

    let config = vivs_config.unwrap();
    let mut port = connection.port;
    let address = connection.address;

    let args = Cli::parse();
    if args.port.is_some() {
        port = args.port.unwrap();
    }

    info!("Attempting to bind on port {port}");

    // Bind/assign the address to the socket (ip address + port number)
    let tcp_listener = TcpListener::bind(format!("{}:{}", address, port))
        .await
        .map_err(|err| {
            error!("TCP listener failed to bind: {err}");
            err
        })?;

    let listener = Listener::new(tcp_listener, DataStore::new());
    listener.run().await?;

    Ok(())
}
