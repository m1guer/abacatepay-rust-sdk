use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum BillingStatus {
    Pending,
    Expired,
    Cancelled,
    Paid,
    Refunded,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BillingMethods {
    #[serde(rename = "PIX")]
    Pix,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BillingKind {
    #[serde(rename = "ONE_TIME")]
    OneTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub product_id: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerMetadata {
    pub name: Option<String>,
    pub cellphone: Option<String>,
    pub email: String,
    pub tax_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub _id: String,
    pub metadata: CustomerMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Billing {
    pub _id: String,
    pub url: String,
    pub amount: f64,
    pub status: BillingStatus,
    #[serde(rename = "devMode")]
    pub dev_mode: bool,
    pub methods: Vec<BillingMethods>,
    pub products: Vec<Product>,
    pub frequency: BillingKind,
    #[serde(rename = "nextBilling")]
    pub next_billing: Option<String>,
    pub customer: Option<Customer>,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "storeId")]
    pub store_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CreateBillingProduct {
    #[serde(rename = "externalId")]
    pub external_id: String,
    pub name: String,
    pub quantity: i32,
    pub price: f64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateBillingData {
    pub frequency: BillingKind,
    pub methods: Vec<BillingMethods>,
    pub products: Vec<CreateBillingProduct>,
    #[serde(rename = "returnUrl")]
    pub return_url: String,
    #[serde(rename = "completionUrl")]
    pub completion_url: String,
    #[serde(rename = "customerId")]
    pub customer_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CreateBillingResponse {
    Success { error: Option<()>, billing: Billing },
    Error { error: String },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ListBillingResponse {
    Success {
        error: Option<()>,
        billings: Vec<Billing>,
    },
    Error {
        error: String,
    },
}
