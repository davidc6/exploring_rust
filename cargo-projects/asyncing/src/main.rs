use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request failed: {0}")]
    RequestError(reqwest::Error),
    #[error("JSON parsing error: {0}")]
    JsonParserError(reqwest::Error),
}

const BOOKS_URL: &str = "https://raw.githubusercontent.com/davidc6/exploring_rust/refs/heads/main/cargo-projects/asyncing/src/mock-data/books.json";

async fn api_call() -> Result<Vec<BookData>, ApiError> {
    let response = reqwest::get(BOOKS_URL)
        .await
        .map_err(ApiError::RequestError)?;

    let data = response
        .json::<EndpointResponse>()
        .await
        .map_err(ApiError::JsonParserError)?;

    Ok(data.data)
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

#[tokio::main]
async fn main() {
    let books = api_call().await.unwrap_or_else(|e| {
        println!("{:?}", e);
        Vec::default()
    });

    println!("{:#?}", books);
}
