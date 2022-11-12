//! main.rs

use std::net::TcpListener;

use bookreview::run;

// async runtime is loaded on top of the main fn
// and used to drive futures (async computations) to completion
// tokio runtime takes async code in the main fn and runs it
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // if run errors, the error will bubble up
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    run(listener)?.await
}
