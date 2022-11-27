use actix_web::{web, HttpResponse, post, http};
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String
}

// actix-web extractor is used here
#[post("/follows")]
async fn follows(
    form: web::Form<FormData>,
    // enables to retrieve data from the app state
    conn: web::Data<PgPool>
) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO follows (id, email, name, followed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(conn.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

    
}
