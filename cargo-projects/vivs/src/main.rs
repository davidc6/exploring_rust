use env_logger::Env;
use log::{error, info};
use vivs::server;

#[tokio::main]
pub async fn main() -> vivs::GenericResult<()> {
    // If RUST_ENV is not set we print info level or above
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Vivs is starting");

    // Start the server
    server::start().await.map_err(|e| {
        error!("Failed to start Vivs server: {e}");
        e
    })?;

    Ok(())
}
