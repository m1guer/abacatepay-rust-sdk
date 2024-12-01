use crate::error::AbacatePayError;
use crate::models::*;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use tracing::{debug, error, instrument};

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
}

pub struct AbacatePay {
    client: Client,
    api_key: String,
    base_url: String,
}

impl AbacatePay {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.abacatepay.com/v1".to_string(),
        }
    }

    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T, AbacatePayError>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        let response_text = response.text().await?;

        debug!(
            status = status.as_u16(),
            response = response_text.as_str(),
            "Received response"
        );

        // If it's an error status code, try to parse the error message
        if !status.is_success() {
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
                error!(
                    status = status.as_u16(),
                    error = error_response.error.as_str(),
                    "API error response"
                );
                return Err(AbacatePayError::ApiError {
                    status,
                    message: error_response.error,
                });
            }

            return Err(AbacatePayError::UnexpectedResponse {
                status,
                response: response_text,
            });
        }

        match serde_json::from_str::<T>(&response_text) {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                error!(
                    error = ?e,
                    response = response_text.as_str(),
                    "Failed to parse API response"
                );
                Err(AbacatePayError::ParseError {
                    message: e.to_string(),
                    response: response_text,
                })
            }
        }
    }

    #[instrument(skip(self, data))]
    pub async fn create_billing(
        &self,
        data: CreateBillingData,
    ) -> Result<Billing, AbacatePayError> {
        let url = format!("{}/billing/create", self.base_url);

        debug!(
            url = url.as_str(),
            request_data = ?data,
            "Sending create billing request"
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header(
                "User-Agent",
                format!("Rust SDK {}", env!("CARGO_PKG_VERSION")),
            )
            .json(&data)
            .send()
            .await?;

        let result: CreateBillingResponse = self.handle_response(response).await?;

        match result {
            CreateBillingResponse::Success { billing, .. } => {
                debug!(billing_id = ?billing._id, "Successfully created billing");
                Ok(billing)
            }
            CreateBillingResponse::Error { error } => {
                error!(
                    error = error.as_str(),
                    "API returned error in response body"
                );
                Err(AbacatePayError::ApiError {
                    status: StatusCode::OK,
                    message: error,
                })
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn list_billings(&self) -> Result<Vec<Billing>, AbacatePayError> {
        let url = format!("{}/billing/list", self.base_url);

        debug!(url = url.as_str(), "Sending list billings request");

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header(
                "User-Agent",
                format!("Rust SDK {}", env!("CARGO_PKG_VERSION")),
            )
            .send()
            .await?;

        let result: ListBillingResponse = self.handle_response(response).await?;

        match result {
            ListBillingResponse::Success { billings, .. } => {
                debug!(
                    billing_count = billings.len(),
                    "Successfully retrieved billings"
                );
                Ok(billings)
            }
            ListBillingResponse::Error { error } => {
                error!(
                    error = error.as_str(),
                    "API returned error in response body"
                );
                Err(AbacatePayError::ApiError {
                    status: StatusCode::OK,
                    message: error,
                })
            }
        }
    }
}
