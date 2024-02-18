#[cfg(test)]
mod server {
    use std::net::SocketAddr;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
    };
    use vivs::{DataStore, Listener};

    async fn init_server() -> SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind to OS chosen port");
        let address = listener.local_addr().unwrap();

        let db = DataStore::new();

        tokio::spawn(async move {
            let listener = Listener::new(listener, db);
            listener.run().await
        });

        address
    }

    #[tokio::test]
    async fn ping_without_value() {
        let addr = init_server().await;

        let mut stream = TcpStream::connect(addr)
            .await
            .expect("Failed to open a TCP connection");
        stream.write_all(b"*1\r\n$4\r\nPING\r\n").await.unwrap();

        let mut buffer = [0; 7];
        let _ = stream.read_exact(&mut buffer).await;

        assert_eq!(b"+PONG\r\n", &buffer);
    }

    #[tokio::test]
    async fn ping_with_value() {
        let addr = init_server().await;

        let mut stream = TcpStream::connect(addr)
            .await
            .expect("Failed to open a TCP connection");
        stream
            .write_all(b"*2\r\n$4\r\nPING\r\n$2\r\nhi\r\n")
            .await
            .unwrap();

        let mut buffer = [0; 5];
        let _ = stream.read_exact(&mut buffer).await;

        assert_eq!(b"+hi\r\n", &buffer);
    }

    #[tokio::test]
    async fn get_missing_key() {
        let addr = init_server().await;

        let mut stream = TcpStream::connect(addr)
            .await
            .expect("Failed to open a TCP connection");
        stream
            .write_all(b"*2\r\n$3\r\nGET\r\n$5\r\nhello\r\n")
            .await
            .unwrap();

        let mut buffer = [0; 3];
        let _ = stream.read_exact(&mut buffer).await;

        assert_eq!(b"_\r\n", &buffer);
    }

    #[tokio::test]
    async fn set_and_get_key() {
        let addr = init_server().await;

        let mut stream = TcpStream::connect(addr)
            .await
            .expect("Failed to open a TCP connection");
        stream
            .write_all(b"*3\r\n$3\r\nSET\r\n$8\r\ngreeting\r\n$5\r\nhello\r\n")
            .await
            .unwrap();

        let mut buffer = [0; 5];
        let _ = stream.read_exact(&mut buffer).await;

        assert_eq!(b"+OK\r\n", &buffer);

        stream
            .write_all(b"*2\r\n$3\r\nGET\r\n$8\r\ngreeting\r\n")
            .await
            .unwrap();

        let mut buffer = [0; 8];
        let _ = stream.read_exact(&mut buffer).await;

        assert_eq!(b"+hello\r\n", &buffer);
    }

    #[tokio::test]
    async fn set_and_delete_key() {
        let addr = init_server().await;

        let mut stream = TcpStream::connect(addr)
            .await
            .expect("Failed to open a TCP connection");
        stream
            .write_all(b"*3\r\n$3\r\nSET\r\n$8\r\ngreeting\r\n$5\r\nhello\r\n")
            .await
            .unwrap();

        let mut buffer = [0; 5];
        let _ = stream.read_exact(&mut buffer).await;

        assert_eq!(b"+OK\r\n", &buffer);

        stream
            .write_all(b"*2\r\n$6\r\nDELETE\r\n$8\r\ngreeting\r\n")
            .await
            .unwrap();

        let mut buffer = [0; 4];
        let _ = stream.read_exact(&mut buffer).await;

        assert_eq!(b":1\r\n", &buffer);

        stream
            .write_all(b"*2\r\n$3\r\nGET\r\n$8\r\ngreeting\r\n")
            .await
            .unwrap();

        let mut buffer = [0; 3];
        let _ = stream.read_exact(&mut buffer).await;

        assert_eq!(b"_\r\n", &buffer);
    }
}