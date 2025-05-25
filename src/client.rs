use crate::billing::{
    Billing, BillingMethods, CreateBillingData, CreateBillingProduct, CreateBillingResponse,
    CustomerMetadata, ListBillingResponse,
};
use crate::pix_charge::{
    CheckPixStatusData, CheckPixStatusResponse, CreatePixChargeData, PixChargeData,
    PixChargeResponse,
};
use crate::{billing::BillingKind, error::AbacatePayError};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use tracing::{debug, error, instrument};

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
    message: String,
    code: String,
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

pub struct PixChargeBuilder<'a> {
    client: &'a AbacatePay,
    data: CreatePixChargeData,
}

pub struct SimulatePixPaymentBuilder<'a> {
    client: &'a AbacatePay,
    id: String,
}

pub struct CheckPixStatusBuilder<'a> {
    client: &'a AbacatePay,
    id: String,
}

impl AbacatePay {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.abacatepay.com/v1".to_string(),
        }
    }
    pub fn create_simulate_pix_payment(&self, id: String) -> SimulatePixPaymentBuilder {
        SimulatePixPaymentBuilder { client: self, id }
    }
    pub fn check_pix_status(&self, id: String) -> CheckPixStatusBuilder {
        CheckPixStatusBuilder { client: self, id }
    }
    pub fn create_pix_charge(&self) -> PixChargeBuilder {
        PixChargeBuilder {
            client: self,
            data: CreatePixChargeData {
                amount: 0,
                expires_in: None,
                description: None,
                customer: None,
            },
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
            ListBillingResponse::Error {
                error,
                code,
                message,
            } => {
                error!(
                    error = error.as_str(),
                    "API returned error in response body"
                );
                Err(AbacatePayError::ApiError {
                    status: StatusCode::OK,
                    code: code,
                    error: error,
                    message: message,
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
                    code: error_response.code,
                    error: error_response.error,
                    message: error_response.message,
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
impl CheckPixStatusBuilder<'_> {
    pub fn id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    #[instrument(skip(self))]
    pub async fn build(self) -> Result<CheckPixStatusData, AbacatePayError> {
        let url = format!("{}/pixQrCode/check", self.client.base_url);
        let response = self
            .client
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.client.api_key))
            .header(
                "User-Agent",
                format!("Rust SDK {}", env!("CARGO_PKG_VERSION")),
            )
            .query(&["id", &self.id])
            .send()
            .await?;
        let result: CheckPixStatusResponse = self.client.handle_response(response).await?;
        match result {
            CheckPixStatusResponse::Success { data, .. } => {
                debug!(pix_charge_id = ?data.status, "Successfully get the status of the PIX payment");
                Ok(data)
            }
            CheckPixStatusResponse::Error {
                error,
                code,
                message,
            } => {
                error!(
                    error = error.as_str(),
                    "API returned error in response body"
                );
                Err(AbacatePayError::ApiError {
                    status: StatusCode::OK,
                    code,
                    error,
                    message,
                })
            }
        }
    }
}

impl SimulatePixPaymentBuilder<'_> {
    pub fn id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    #[instrument(skip(self))]
    pub async fn build(self) -> Result<PixChargeData, AbacatePayError> {
        let url = format!("{}/pixQrCode/simulate-payment", self.client.base_url);
        let response = self
            .client
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.client.api_key))
            .header(
                "User-Agent",
                format!("Rust SDK {}", env!("CARGO_PKG_VERSION")),
            )
            .query(&["id", &self.id])
            .send()
            .await?;
        let result: PixChargeResponse = self.client.handle_response(response).await?;
        match result {
            PixChargeResponse::Success { data, .. } => {
                debug!(pix_charge_id = ?data.amount, "Successfully simulated PIX payment");
                Ok(data)
            }
            PixChargeResponse::Error {
                error,
                code,
                message,
            } => {
                error!(
                    error = error.as_str(),
                    "API returned error in response body"
                );
                Err(AbacatePayError::ApiError {
                    status: StatusCode::OK,
                    error: error,
                    message: message,
                    code: code,
                })
            }
        }
    }
}
impl PixChargeBuilder<'_> {
    pub fn amount(mut self, amount: i64) -> Self {
        self.data.amount = amount;
        self
    }

    pub fn expires_in(mut self, expires_in: Option<u64>) -> Self {
        self.data.expires_in = expires_in;
        self
    }
    pub fn description(mut self, description: Option<String>) -> Self {
        self.data.description = description;
        self
    }
    pub fn customer(mut self, customer: Option<CustomerMetadata>) -> Self {
        self.data.customer = customer;
        self
    }

    #[instrument(skip(self))]
    pub async fn build(self) -> Result<PixChargeData, AbacatePayError> {
        let url = format!("{}/pixQrCode/create", self.client.base_url);
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
        let result: PixChargeResponse = self.client.handle_response(response).await?;
        match result {
            PixChargeResponse::Success { data, .. } => {
                debug!(pix_charge_id = ?data.amount, "Successfully created PIX charge");
                Ok(data)
            }
            PixChargeResponse::Error {
                error,
                code,
                message,
            } => {
                error!(
                    error = error.as_str(),
                    "API returned error in response body"
                );
                Err(AbacatePayError::ApiError {
                    status: StatusCode::OK,
                    code,
                    error,
                    message,
                })
            }
        }
    }
}

impl BillingBuilder<'_> {
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
            CreateBillingResponse::Error {
                error,
                code,
                message,
            } => {
                error!(
                    error = error.as_str(),
                    "API returned error in response body"
                );
                Err(AbacatePayError::ApiError {
                    status: StatusCode::OK,
                    code,
                    error,
                    message,
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

    #[test]
    async fn create_pix_charge_builder() {
        let client = client();

        let builder = client
            .create_pix_charge()
            .amount(1000)
            .expires_in(Some(3600))
            .description(Some("Test PIX charge".to_string()));

        assert_eq!(builder.data.amount, 1000);
        assert_eq!(builder.data.expires_in, Some(3600));
        assert_eq!(
            builder.data.description,
            Some("Test PIX charge".to_string())
        );
        assert!(builder.data.customer.is_none());
    }

    #[test]
    async fn pix_charge_with_customer() {
        let client = client();

        let customer = CustomerMetadata {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            tax_id: "123.456.789-00".to_string(),
            cellphone: "5511999999999".to_string(),
        };

        let builder = client.create_pix_charge().customer(Some(customer.clone()));

        // Check customer data is correctly stored in the builder
        let builder_customer = builder.data.customer.as_ref().unwrap();
        assert!(builder_customer.name == customer.name);
        assert!(builder_customer.email == customer.email);
        assert!(builder_customer.tax_id == customer.tax_id);
        assert!(builder_customer.cellphone == customer.cellphone);
    }

    #[test]
    async fn pix_charge_default_values() {
        let client = client();
        let builder = client.create_pix_charge();

        assert_eq!(builder.data.amount, 00);
        assert_eq!(builder.data.expires_in, None);
        assert_eq!(builder.data.description, None);
        assert!(builder.data.customer.is_none());
    }

    #[test]
    async fn simulate_pix_payment_builder() {
        let client = client();

        let builder = client.create_simulate_pix_payment("test-charge-id".to_string());
        assert_eq!(builder.id, "test-charge-id");

        // Test the setter method
        let builder = builder.id("new-test-id".to_string());
        assert_eq!(builder.id, "new-test-id");
    }
}
