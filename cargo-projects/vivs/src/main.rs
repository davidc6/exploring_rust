use log::{error, info};
use serde::Deserialize;
use std::{env, fs};
use vivs::server;

#[derive(Deserialize, Debug)]
struct Data {
    connection: Config,
}

#[derive(Deserialize, Debug)]
struct Config {
    address: String,
    port: u16,
}

#[tokio::main]
pub async fn main() -> vivs::GenericResult<()> {
    env_logger::init();

    // TODO: load config
    let current_dir = env::current_dir()?;
    let config_dir = current_dir.join("config/config.toml");
    let file_contents = fs::read_to_string(config_dir)?;
    let config: Data = toml::from_str(&file_contents)?;

    info!("Vivs is starting");

    server::start(&config.connection.address, config.connection.port)
        .await
        .map_err(|e| {
            error!("Failed to start Vivs server: {e}");
            e
        })?;

    Ok(())
}
