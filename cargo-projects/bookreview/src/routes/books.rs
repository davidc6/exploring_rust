use actix_web::{HttpResponse, web, post, http};
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
struct FormData {
    title: String,
    author: String
}

// actix-web extractor (web::Data) is used here which enables access to request information
#[post("/books")]
async fn books(
    form: web::Form<FormData>,
    // enables to retrieve data from the app state
    connection: web::Data<PgPool>
) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO books (id, title, author, added_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.title,
        form.author,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
