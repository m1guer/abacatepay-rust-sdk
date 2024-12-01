use crate::error::AbacatePayError;
use crate::models::*;
use reqwest::Client;

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

    pub async fn create_billing(
        &self,
        data: CreateBillingData,
    ) -> Result<Billing, AbacatePayError> {
        let response = self
            .client
            .post(format!("{}/billing", self.base_url))
            .header("authorization", format!("Bearer {}", self.api_key))
            .json(&data)
            .send()
            .await?
            .json::<CreateBillingResponse>()
            .await?;
        println!("{}/billing", self.base_url);
        match response {
            CreateBillingResponse::Success { billing, .. } => Ok(billing),
            CreateBillingResponse::Error { error } => Err(AbacatePayError::ApiError(error)),
        }
    }

    pub async fn list_billings(&self) -> Result<Vec<Billing>, AbacatePayError> {
        let response = self
            .client
            .get(format!("{}/billing/list", self.base_url))
            .header("authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<ListBillingResponse>()
            .await?;

        match response {
            ListBillingResponse::Success { billings, .. } => Ok(billings),
            ListBillingResponse::Error { error } => Err(AbacatePayError::ApiError(error)),
        }
    }
}
