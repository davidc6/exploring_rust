use log::{error, info};
use std::{env, fs};
use vivs::{server, Config, VIVS_CONFIG};

#[tokio::main]
pub async fn main() -> vivs::GenericResult<()> {
    env_logger::init();

    info!("Vivs is starting");

    // Load the config up
    let config_dir = env::current_dir()?.join("config/config.toml");
    let file_contents_as_string = fs::read_to_string(config_dir)?;
    let config: Config = toml::from_str(&file_contents_as_string)?;
    VIVS_CONFIG.set(config)?;

    // Start the server
    server::start().await.map_err(|e| {
        error!("Failed to start Vivs server: {e}");
        e
    })?;

    Ok(())
}
