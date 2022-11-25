use actix_web::{get, web, Result, Responder};
use serde::{Serialize};

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