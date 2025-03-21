///  Tokio an event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.
///
/// Async functions are lazy in Rust, they need to be asked to do work.
///
/// Good resource to learn more about different runtimes https://corrode.dev/blog/async/
///
use tokio::net::TcpListener;

/// This function returns a Future which
/// is a type that represents computation that
/// msy complete later. These implement
/// Future trait.
async fn bind_random() -> TcpListener {
    TcpListener::bind("127.1.1.1").await.unwrap()
}

async fn run() {
    // Wait for bind_random() to complete and
    // then return control to the caller.
    // Control is yielded to the async runtime (async executor),
    // that is responsible (make sure progress is being made,
    // balance task resources) for managing all the async tasks.
    // Such runtimes are tokio, async-std (now discontinued), smol.
    let listener = bind_random().await;
}

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    // keep on processing requests hence the loop here
    loop {
        let (mut connection, _) = listener.accept().await?;

        // spawn to hand off task to executor without waiting for it to complete
        tokio::spawn(async move {
            let (mut stream_reader, mut stream_writer) = connection.split();
            tokio::io::copy(&mut stream_reader, &mut stream_writer)
                .await
                .unwrap();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let requests = vec!["hello", "world", "foo", "bar"];

        tokio::spawn(echo(listener));

        // let request = "hello";

        for request in requests {
            let mut s = tokio::net::TcpStream::connect(addr).await;

            let mut socket = s.unwrap();
            let (mut reader, mut writer) = socket.split();

            // Send the request
            writer.write_all(request.as_bytes()).await.unwrap();
            // Close the write side of the socket
            writer.shutdown().await.unwrap();

            // Read the response
            let mut buf = Vec::with_capacity(request.len());
            reader.read_to_end(&mut buf).await.unwrap();
            assert_eq!(&buf, request.as_bytes());
        }
    }
}
