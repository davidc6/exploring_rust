use log::{error, info};
use vivs::server;

#[tokio::main]
pub async fn main() -> vivs::GenericResult<()> {
    env_logger::init();

    info!("Vivs is starting");

    // Start the server
    server::start().await.map_err(|e| {
        error!("Failed to start Vivs server: {e}");
        e
    })?;

    Ok(())
}
