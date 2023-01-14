use tokio::{net::{TcpListener, TcpStream}, io::{BufReader}};
use tokio::io::AsyncBufReadExt;

async fn handle_stream(mut stream: TcpStream, addr: std::net::SocketAddr) {
    let (read, write) = stream.split();
    let mut read_buffer = BufReader::new(read);

    loop {
        let mut buffer = String::new();
        let x = read_buffer.read_line(&mut buffer).await;

        match x {
            Ok(val) => {
                match write.try_write(b"+PONG\r\n") {
                    Ok(val) => {
                        println!("{:?}", buffer);
                    }
                    Err(e) => {

                    }
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }

        buffer.clear();
    }
}

pub async fn start(addr: String, port: String) -> std::io::Result<()> {
    let location = format!("{}:{}", addr, port);
    let listener = TcpListener::bind(&location).await?;

    loop {
        let (mut stream, addr) = listener.accept().await?; // TODO - checks?

        tokio::spawn(async move {
            handle_stream(stream, addr).await;
        });
    }
}
