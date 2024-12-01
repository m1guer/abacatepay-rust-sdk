use thiserror::Error;

#[derive(Error, Debug)]
pub enum AbacatePayError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("API error: {0}")]
    ApiError(String),
}
