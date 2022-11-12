//! tests/health_check.rs

#[tokio::test]
async fn ping_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:7878/ping")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

fn spawn_app() {
    let server = bookreview::run().expect("Failed to bind address");
    // Server gets launched as a background task
    // let is non-binding hence no use for it here
    let _ = tokio::spawn(server);
}
