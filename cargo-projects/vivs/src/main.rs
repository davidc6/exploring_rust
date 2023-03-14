use vivs::server;

#[tokio::main]
pub async fn main() -> vivs::Result<()> {
    let addr = "127.0.0.1".to_string(); // TODO - extract into a config file
    let port = "6379".to_string();

    server::start(addr, port).await?;

    Ok(())
}
