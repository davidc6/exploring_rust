use actix_web::{web, HttpResponse, post};

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String
}

// actix-web extractor is used here
#[post("/follows")]
async fn follows(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
