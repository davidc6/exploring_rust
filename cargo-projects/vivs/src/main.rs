use tokio::{net::{TcpListener, TcpStream}, io::BufReader};
use std::{io::{self}};

// async fn process_socket<T>(stream: TcpStream) {
//     let mut reader = BufReader::new(stream);
//     let mut buffer = String::new();

//     loop {
//         tokio::select! {
//             // read_result = reader.read_line(&mut buffer) => {
//                 // chat message received!
//             // }
//         }
//     }
// }

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let listener = TcpListener::bind("localhost:6379").await?;

//     loop {
//         let (socket, _) = listener.accept().await?;

//         tokio::spawn(async move {
//             process_socket(socket).await;
//         });
//     }

// }

mod server;

#[tokio::main]
async fn main() {
    let addr = "localhost".to_string();
    let port = "6379".to_string();

    server::start(addr, port).await;
}
