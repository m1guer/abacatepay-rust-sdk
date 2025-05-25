use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AbacatePayError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    ApiError {
        status: StatusCode,
        error: String,
        message: String,
        code: String,
    },

    #[error("Failed to parse API response: {message}. Response: {response}")]
    ParseError { message: String, response: String },

    #[error("Unexpected response ({status}): {response}")]
    UnexpectedResponse {
        status: StatusCode,
        response: String,
    },
}
