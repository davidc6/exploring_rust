use tokio::{net::{TcpListener, TcpStream}, io::BufReader};
use tokio::io::AsyncBufReadExt;

async fn handle_stream(stream: TcpStream, addr: std::net::SocketAddr) {
    let mut r = BufReader::new(stream);
    let mut b = String::new();

    loop {
        tokio::select! {
            read_result = r.read_line(&mut b) => {

            }
        }
    }
}

pub async fn start(addr: String, port: String) {
    let location = format!("{}:{}", addr, port);
    let listener = TcpListener::bind(&location).await.expect("Failed to bind to addr");

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_stream(stream, addr).await;
        });
    }
}