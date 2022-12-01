//! tests/health_check.rs

use std::{net::TcpListener};
use bookreview::configuration::{configuration, DbSettings};
use sqlx::{PgConnection, Connection, PgPool, Executor};
use uuid::Uuid;

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
        .post(format!("{}/follows", &test_app.addr))
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

        assert_eq!(saved.email, "john.doe@gmail.com");
        assert_eq!(saved.name, "john doe");
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

#[derive(Debug, Clone)]
pub struct TestApp {
    pub addr: String,
    pub db_name: String,
    pub db_pool: PgPool
}

impl TestApp {
    async fn drop_db(&mut self) {
        self.db_pool.close().await;

        let conf = configuration().expect("Failed to read the config");
        let mut conn = PgConnection::connect(&conf.database.conn_str())
            .await
            .expect("Could not connect to DB");

        conn
            .execute(
                format!(
                    r#"
                    SELECT pg_terminate_backend(pg_stat_activity.pid)
                    FROM pg_stat_activity
                    WHERE pg_stat_activity.datname = '{}'
                    AND pid <> pg_backend_pid()
                    "#,
                    self.db_name
                )
                .as_str(),
            )
            .await
            .expect("Failed to terminate current connections to test db");

        let a = conn
            .execute(format!(
                r#"
                SELECT * FROM pg_stat_activity WHERE datname = '{}'
                "#,
                self.db_name
            )
            .as_str(),
        )
            .await
            .expect("Failed to query DB");

        println!("{:?}", a);

        conn
            .execute(format!(
                r#"
                DROP DATABASE "{}";
                "#,
                self.db_name)
                .as_str()
            )
            .await
            .expect("Failed to drop a db");
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        // since async traits are not yet supported in Rust,
        // we spawn a thread to drop a database after each test
        std::thread::scope(|s| {
            s.spawn(|| {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                runtime.block_on(self.drop_db());
            });
        });
    }
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");

    // OS assigned port address
    let port = listener.local_addr().unwrap().port();

    // Return application address to the caller
    let addr = format!("http://127.0.0.1:{}", port);

    let mut conf = configuration().expect("Failed to read the config");
    conf.database.db_name = Uuid::new_v4().to_string(); // generate random DB name
    let conn_pool = conf_db(&conf.database)
        .await;

    let server = bookreview::startup::run(listener, conn_pool.clone())
        .expect("Failed to bind address");

    // Server gets launched as a background task
    // let is non-binding hence no use for it here
    let _ = tokio::spawn(server);

    TestApp {
        addr,
        db_name: conf.database.db_name,
        db_pool: conn_pool
    }
}

pub async fn conf_db(config: &DbSettings) -> PgPool {
    let mut conn = PgConnection::connect(&config.conn_str_without_db())
        .await
        .expect("Failed to connect to Postgres");
    conn
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.db_name).as_str())
        .await
        .expect("Failed to create database");

    // Migration
    let conn_pool = PgPool::connect(&config.conn_str())
        .await
        .expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("Failed to migrate");

    conn_pool
}