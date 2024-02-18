use log::{error, info};
use vivs::server;

#[tokio::main]
pub async fn main() -> vivs::GenericResult<()> {
    env_logger::init();

    let ipv4 = "127.0.0.1".to_string(); // TODO - extract into a config file
    let port = "6379".to_string();

    info!("Vivs is starting");

    server::start(ipv4, port).await.map_err(|e| {
        error!("Failed to start Vivs server: {e}");
        e
    })?;

    Ok(())
}
