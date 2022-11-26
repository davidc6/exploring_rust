//! main.rs
use std::{net::TcpListener, collections::HashMap};
use bookreview::{startup::run, configuration::{configuration, DbSettings, Settings}};

// async runtime is loaded on top of the main fn
// and used to drive futures (async computations) to completion
// tokio runtime takes async code in the main fn and runs it
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration().expect("Failed to read configuration.");

    // if run errors, the error will bubble up
    let addr = format!("127.0.0.1:{}", configuration.app_port);

    let listener = TcpListener::bind(addr).expect("Failed to bind random port");
    run(listener)?.await
}
