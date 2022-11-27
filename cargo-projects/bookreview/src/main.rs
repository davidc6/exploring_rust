//! main.rs
use std::{net::TcpListener};
use bookreview::{startup::run, configuration::{configuration}};
use sqlx::{PgPool};

// async runtime is loaded on top of the main fn
// and used to drive futures (async computations) to completion
// tokio runtime takes async code in the main fn and runs it
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration().expect("Failed to read configuration.");

    let conn_pool = PgPool::connect(&configuration.database.conn_str())
        .await
        .expect("Failed to connect to Postgres.");

    // if run errors, the error will bubble up
    let addr = format!("127.0.0.1:{}", configuration.app_port);

    let listener = TcpListener::bind(addr).expect("Failed to bind random port");
    run(listener, conn_pool)?.await
}
