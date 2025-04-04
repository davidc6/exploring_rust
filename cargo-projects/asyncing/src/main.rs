use std::fmt;

use axum::{
    body::Body,
    debug_handler,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    serve::Listener,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request failed: {0}")]
    RequestError(reqwest::Error),
    #[error("JSON parsing error: {0}")]
    JsonParserError(reqwest::Error),
}

const BOOKS_URL: &str = "https://raw.githubusercontent.co/davidc6/exploring_rust/refs/heads/main/cargo-projects/asyncing/src/mock-data/books.json";

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
    data: Vec<BookData>,
    status: String,
}

#[debug_handler]
async fn get_books() -> (StatusCode, Response) {
    let response = reqwest::get(BOOKS_URL).await;

    let Ok(res) = response else {
        return (
            StatusCode::BAD_REQUEST,
            Json(OkResponse {
                status: ResponseStatus::Error.to_string(),
                data: vec![],
            })
            .into_response(),
            // Body::new(Json(OkResponse {
            //     data: [],
            //     status: ResponseStatus::Fail.to_string(),
            // }))
            // .into_response(),
        );
        // return (StatusCode::BAD_REQUEST, Body::empty()).into_response();
    };

    let r = res.json::<EndpointResponse>().await.unwrap();
    (
        StatusCode::OK,
        Json(OkResponse {
            status: ResponseStatus::Success.to_string(),
            data: r.data,
        })
        .into_response(),
    )
}

// let data = response
//     .json::<EndpointResponse>()
//     .await
//     .map_err(ApiError::JsonParserError)?;

// Ok(data.data)

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

#[derive(Debug, Serialize, Deserialize)]
struct EndpointError {}

async fn root() -> String {
    "Hello, world!\n".to_owned()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/books", get(get_books));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
