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

pub struct BillingBuilder<'a> {
    client: &'a AbacatePay,
    data: CreateBillingData,
}

impl AbacatePay {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.abacatepay.com/v1".to_string(),
        }
    }

    pub fn create_billing(&self) -> BillingBuilder {
        BillingBuilder {
            client: self,
            data: CreateBillingData {
                frequency: BillingKind::OneTime,
                methods: vec![],
                products: vec![],
                return_url: String::new(),
                completion_url: String::new(),
                customer_id: None,
            },
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
}

impl<'a> BillingBuilder<'a> {
    pub fn frequency(mut self, frequency: BillingKind) -> Self {
        self.data.frequency = frequency;
        self
    }

    pub fn method(mut self, method: BillingMethods) -> Self {
        self.data.methods.push(method);
        self
    }

    pub fn product(mut self, product: CreateBillingProduct) -> Self {
        self.data.products.push(product);
        self
    }

    pub fn return_url(mut self, url: String) -> Self {
        self.data.return_url = url;
        self
    }

    pub fn completion_url(mut self, url: String) -> Self {
        self.data.completion_url = url;
        self
    }

    pub fn customer_id(mut self, id: String) -> Self {
        self.data.customer_id = Some(id);
        self
    }

    #[instrument(skip(self))]
    pub async fn build(self) -> Result<Billing, AbacatePayError> {
        let url = format!("{}/billing/create", self.client.base_url);

        debug!(
            url = url.as_str(),
            request_data = ?self.data,
            "Sending create billing request"
        );

        let response = self
            .client
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.client.api_key))
            .header(
                "User-Agent",
                format!("Rust SDK {}", env!("CARGO_PKG_VERSION")),
            )
            .json(&self.data)
            .send()
            .await?;

        let result: CreateBillingResponse = self.client.handle_response(response).await?;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    fn client() -> AbacatePay {
        AbacatePay::new("my-key!".to_string())
    }

    #[test]
    async fn create_billing_builder() {
        let client = client();

        let builder = client
            .create_billing()
            .frequency(BillingKind::OneTime)
            .method(BillingMethods::Pix)
            .customer_id("my-customer-id".to_string());

        assert_eq!(builder.data.frequency, BillingKind::OneTime);
        assert_eq!(builder.data.methods, vec![BillingMethods::Pix]);
        assert_eq!(builder.data.customer_id, Some("my-customer-id".to_string()));
    }

    #[test]
    async fn payment_method() {
        let client = client();

        let builder = client.create_billing().method(BillingMethods::Pix);

        assert_eq!(builder.data.methods, vec![BillingMethods::Pix]);
    }

    #[test]
    async fn billing_urls() {
        let client = client();

        let builder = client
            .create_billing()
            .return_url("http://localhost:3030".to_string())
            .completion_url("http://localhost:3030?success=true".to_string());

        assert_eq!(builder.data.return_url, "http://localhost:3030");
        assert_eq!(
            builder.data.completion_url,
            "http://localhost:3030?success=true"
        );
    }

    #[test]
    async fn billing_products() {
        let client = client();

        let product1 = CreateBillingProduct {
            external_id: "external_id_1".to_string(),
            name: "Product 1".to_string(),
            price: 10.0,
            quantity: 1,
            description: None,
        };

        let product2 = CreateBillingProduct {
            external_id: "external_id_2".to_string(),
            name: "Product 2".to_string(),
            price: 24.12,
            quantity: 2,
            description: Some("product_description".to_string()),
        };

        let builder = client
            .create_billing()
            .product(product1.clone())
            .product(product2.clone());

        assert_eq!(builder.data.products, vec![product1, product2]);
    }

    #[test]
    async fn default_values() {
        let client = client();
        let builder = client.create_billing();

        assert_eq!(builder.data.frequency, BillingKind::OneTime);
        assert!(builder.data.methods.is_empty());
        assert!(builder.data.products.is_empty());
        assert_eq!(builder.data.return_url, String::new());
        assert_eq!(builder.data.completion_url, String::new());
        assert_eq!(builder.data.customer_id, None);
    }
}
