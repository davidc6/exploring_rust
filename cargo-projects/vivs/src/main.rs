use log::info;
use vivs::server;

#[tokio::main]
pub async fn main() -> vivs::Result<()> {
    env_logger::init();

    let ipv4 = "127.0.0.1".to_string(); // TODO - extract into a config file
    let port = "6379".to_string();

    info!("Vivs is starting");

    server::start(ipv4, port).await?;

    Ok(())
}
