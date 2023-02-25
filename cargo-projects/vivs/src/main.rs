use vivs::server;

#[tokio::main]
pub async fn main() -> vivs::Result<()> {
    let addr = "127.0.0.1".to_string();
    let port = "6379".to_string();

    // Original
    // server::start(addr, port).await?;

    // Improvement - splitting functionality into mods
    server::run(addr, port).await?;

    Ok(())
}
