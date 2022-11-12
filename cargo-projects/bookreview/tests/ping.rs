//! tests/health_check.rs

use std::net::TcpListener;

#[tokio::test]
async fn ping_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/ping", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // OS assigned port address
    let port = listener.local_addr().unwrap().port();

    let server = bookreview::run(listener).expect("Failed to bind address");

    // Server gets launched as a background task
    // let is non-binding hence no use for it here
    let _ = tokio::spawn(server);

    // Return application address to the caller
    format!("http://127.0.0.1:{}", port)
}
