//! tests/health_check.rs

use std::{net::TcpListener, fmt::format};

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
    assert_eq!(Some(15), response.content_length());
}

#[tokio::test]
async fn follow_returns_200_when_valid_data() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=john%20doe&email=john.doe%40gmail.com";
    let res = client
        .post(format!("{}/follows", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, res.status().as_u16());
}

#[tokio::test]
async fn follow_returns_400_when_missing_data() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    // table-driven / parametised test
    let cases = vec![
        ("name=john%doe", "missing email"),
        ("email=john.doe%40gmail.com", "missing name"),
        ("", "missing name and email")
    ];

    for (body, err_msg) in cases {
        let res = client
            .post(format!("{}/follows", addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            res.status(),
            "The endpoint should have failed with 400 when the payload was {}",
            err_msg
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // OS assigned port address
    let port = listener.local_addr().unwrap().port();

    let server = bookreview::startup::run(listener).expect("Failed to bind address");

    // Server gets launched as a background task
    // let is non-binding hence no use for it here
    let _ = tokio::spawn(server);

    // Return application address to the caller
    format!("http://127.0.0.1:{}", port)
}
