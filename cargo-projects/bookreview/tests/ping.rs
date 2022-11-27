//! tests/health_check.rs
use std::{net::TcpListener};
use bookreview::configuration::{configuration};
use sqlx::{PgConnection, Connection, PgPool};

#[tokio::test]
async fn ping_works() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/ping", test_app.addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(15), response.content_length());
}

#[tokio::test]
async fn follow_returns_200_when_valid_data() {
    let test_app = spawn_app().await;
    let conf = configuration().expect("Failed to get the config");
    let conn_str = conf.database.conn_str();

    let mut conn = PgConnection::connect(&conn_str)
        .await
        .expect("Failed to connect to Postgres database");

    let client = reqwest::Client::new();

    let body = "name=john%20doe&email=john.doe%40gmail.com";
    let res = client
        .post(format!("{}/follows", test_app.addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, res.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM follows")
        .fetch_one(&mut conn)
        .await
        .expect("Failed to fetch saved follows.");

    assert_eq!(saved.email, "a@a.com");
    assert_eq!(saved.name, "a");
}

#[tokio::test]
async fn follow_returns_400_when_missing_data() {
    let addr = spawn_app().await;
    let client = reqwest::Client::new();
    // table-driven / parametised test
    let cases = vec![
        ("name=john%doe", "missing email"),
        ("email=john.doe%40gmail.com", "missing name"),
        ("", "missing name and email")
    ];

    for (body, err_msg) in cases {
        let res = client
            .post(format!("{}/follows", addr.addr))
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

#[derive(Debug)]
pub struct TestApp {
    pub addr: String,
    pub db_pool: PgPool
} 

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    // OS assigned port address
    let port = listener.local_addr().unwrap().port();

    // Return application address to the caller
    let addr = format!("http://127.0.0.1:{}", port);

    let conf = configuration().expect("Failed to read the config");
    let conn_pool = PgPool::connect(&conf.database.conn_str())
        .await
        .expect("Failed to connect to PG");


    let server = bookreview::startup::run(listener, conn_pool.clone())
        .expect("Failed to bind address");

    // Server gets launched as a background task
    // let is non-binding hence no use for it here
    let _ = tokio::spawn(server);

    TestApp {
        addr,
        db_pool: conn_pool
    }

}
