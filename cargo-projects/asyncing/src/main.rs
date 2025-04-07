use axum::{
    debug_handler,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request failed: {0}")]
    RequestError(reqwest::Error),
    #[error("JSON parsing error: {0}")]
    JsonParserError(reqwest::Error),
}

const BOOKS_URL: &str = "https://raw.githubusercontent.com/davidc6/exploring_rust/refs/heads/main/cargo-projects/asyncing/src/mock-data/books.json";

#[derive(Deserialize, Serialize)]
enum ResponseStatus {
    Success,
    Fail,
    Error,
}

impl fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseStatus::Success => write!(f, "success"),
            ResponseStatus::Fail => write!(f, "fail"),
            ResponseStatus::Error => write!(f, "error"),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct OkResponse {
    status: String,
    data: Vec<BookData>,
}

#[derive(Deserialize, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BookData {
    id: String,
    name: String,
    year: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EndpointResponse {
    data: Vec<BookData>,
}

async fn root() -> String {
    "Hello, world!\n".to_owned()
}

#[debug_handler]
async fn list_books() -> Response {
    let response = reqwest::get(BOOKS_URL).await;

    match response {
        Ok(res) => (
            StatusCode::OK,
            Json(OkResponse {
                status: ResponseStatus::Success.to_string(),
                data: res.json::<EndpointResponse>().await.unwrap().data,
            }),
        )
            .into_response(),
        Err(e) => {
            // TODO: add tracing
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    status: ResponseStatus::Error.to_string(),
                    message: "There has been an error with the request".to_owned(),
                }),
            )
                .into_response()
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/books", get(list_books));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
