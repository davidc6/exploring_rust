use std::panic;

///  Tokio an event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications.
///
/// Async functions are lazy in Rust, they need to be asked to do work.
///
/// Good resource to learn more about different runtimes https://corrode.dev/blog/async/
///
/// Another things to note is the difference between std spawn and tokio spawn.
/// std::thread::spawn delegates control to OS scheduler
/// tokio::spawn delegates the control to the async executor in the user space,
/// meaning that OS scheduler is not involved in deciding which task to run next.
///
use tokio::{join, net::TcpListener, task::JoinHandle};

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

async fn log() {
    println!("Logging");
}

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    // keep on processing requests hence the loop here
    loop {
        let (mut connection, _) = listener.accept().await?;
        // spawn to hand off task to executor without waiting for it to complete
        // task is an async green thread.
        // tokio continues running the spawned task in background
        // with task that spawned it concurrently
        // Here we have an async block instead of a function.
        // If the spawned process panics, the panic is called by the executor
        // Spawn a task
        tokio::spawn(async move {
            let (mut stream_reader, mut stream_writer) = connection.split();
            tokio::io::copy(&mut stream_reader, &mut stream_writer)
                .await
                .unwrap();
        });
    }
}

pub async fn echoes(listener: TcpListener, listener_2: TcpListener) -> Result<(), anyhow::Error> {
    // ::spawn here to hand off to the executor
    let first = tokio::spawn(echo(listener));
    let second = tokio::spawn(echo(listener_2));

    let (first, second) = join!(first, second);

    first??;
    second??;

    // loop {
    //     let (mut connection, _) = listener.accept().await?;
    //     let (mut connection_2, _) = listener_2.accept().await?;

    //     let (first, second) = join!(
    // spawn to hand off task to executor without waiting for it to complete
    // task is an async green thread.
    // tokio continues running the spawned task in background
    // with task that spawned it concurrently
    // Here we have an async block instead of a function.
    // If the spawned process panics, the panic is called by the executor
    // Spawn a task
    //     tokio::spawn(async move {
    //         let (mut stream_reader, mut stream_writer) = connection.split();
    //         tokio::io::copy(&mut stream_reader, &mut stream_writer)
    //             .await
    //             .unwrap();
    //     }),
    //     tokio::spawn(async move {
    //         let (mut stream_reader, mut stream_writer) = connection_2.split();
    //         tokio::io::copy(&mut stream_reader, &mut stream_writer)
    //             .await
    //             .unwrap();
    //     })
    // );

    // Addition: Do other work
    // log().await;
    // Return to the caller once the telemetry data has been delivered.

    // Handle the error since
    // since it won't be propagated automatically
    // if let Err(e) = first {
    // Check if the task panicked
    // if let Ok(reason) = e.try_into_panic() {
    // The task panicked, unwind the panic,
    // propagate to the current task.
    // panic::resume_unwind(reason);
    // }
    // }

    // if let Err(e) = second {
    // Check if the task panicked
    // if let Ok(reason) = e.try_into_panic() {
    // The task panicked, unwind the panic,
    // propagate to the current task.
    // panic::resume_unwind(reason);
    // }
    //     }
    // }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        tokio::spawn(echoes(first_listener, second_listener));

        let requests = vec!["hello", "world", "foo", "bar"];
        let mut join_set = JoinSet::new();

        for request in requests.clone() {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, mut writer) = socket.split();

                    // Send the request
                    writer.write_all(request.as_bytes()).await.unwrap();
                    // Close the write side of the socket
                    writer.shutdown().await.unwrap();

                    // Read the response
                    let mut buf = Vec::with_capacity(request.len());
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, request.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
