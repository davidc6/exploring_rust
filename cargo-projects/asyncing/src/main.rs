use axum::{
    debug_handler,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use env_logger::{self, Env};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::SocketAddr;
use thiserror::Error;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

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
    info!("Handling list_books request");

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
            error!("{}", e);

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
    // Tracing allows us to record structured events with additional information.
    // "Spans" are the building blocks of tracing that have start and end times, other
    // relevant metadata, may be entered and exited by the flow of execution and may exist
    // with a nested tree of similar spans. A span is a logical unit of work in completing
    // a user request.
    //
    // 1. Logging initialisation
    //
    // Env filter is a layer (a composable handler for tracing events)
    // that filters spans (units of work or operation)
    // and events (structured logs) based on filter directives.
    //
    // A span tracks a specific request operation enabling
    // us to see what happened with a certain timeframe.
    // Example: https://opentelemetry.io/docs/concepts/signals/traces/#spans
    //
    // with_env_filter() - determine if a span or event is enabled by looking at the EnvFilter
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("asyncing=info,tower_http=debug"))
                .unwrap(),
        )
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/books", get(list_books))
        .layer(TraceLayer::new_for_http());

    let address = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Starting server on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
