use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;

#[get("/books")]
async fn books() -> impl Responder {
    "Books endpoint"
}

#[derive(Serialize)]
struct Health {
    status: String,
}

#[get("/ping")]
async fn ping() -> Result<impl Responder> {
    let health = Health {
        status: "OK".to_string()
    };
    Ok(web::Json(health))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(books)
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}
