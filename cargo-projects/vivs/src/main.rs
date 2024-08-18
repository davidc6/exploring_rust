use log::{error, info};
use serde::Deserialize;
use std::{env, fs};
use vivs::server;

#[derive(Deserialize, Debug)]
struct Config {
    connection: Connection,
}

#[derive(Deserialize, Debug)]
struct Connection {
    address: String,
    port: u16,
}

#[tokio::main]
pub async fn main() -> vivs::GenericResult<()> {
    env_logger::init();

    let config_dir = env::current_dir()?.join("config/config.toml");
    let file_contents_as_string = fs::read_to_string(config_dir)?;

    let config: Config = toml::from_str(&file_contents_as_string)?;
    let Config {
        connection: Connection { address, port },
    } = config;

    info!("Vivs is starting");

    server::start(&address, port).await.map_err(|e| {
        error!("Failed to start Vivs server: {e}");
        e
    })?;

    Ok(())
}
