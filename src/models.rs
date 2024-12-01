use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub enum BillingStatus {
    PENDING,
    EXPIRED,
    CANCELLED,
    PAID,
    REFUNDED,
}
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum BillingMethods {
    PIX,
}
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum BillingKind {
    ONE_TIME,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Metadata {
    pub fee: i64,
    #[serde(rename = "returnUrl")]
    pub return_url: String,
    #[serde(rename = "completionUrl")]
    pub completion_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Product {
    #[serde(rename = "productId")]
    pub product_id: String,
    pub quantity: i64,
}
#[derive(Debug, Deserialize, Clone)]
pub struct CustomerMetadata {
    pub name: String,
    pub cellphone: String,
    #[serde(rename = "taxId")]
    pub tax_id: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomerId {
    pub metadata: CustomerMetadata,
    pub _id: String,
    #[serde(rename = "publicId")]
    pub public_id: String,
    #[serde(rename = "storeId")]
    pub store_id: String,
    #[serde(rename = "devMode")]
    pub dev_mode: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub __v: i64,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Customer {
    pub _id: String,
    pub metadata: CustomerMetadata,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Billing {
    pub metadata: Metadata,
    pub _id: String,
    #[serde(rename = "publicId")]
    pub public_id: String,
    pub products: Vec<Product>,
    pub amount: i64,
    pub status: BillingStatus,
    #[serde(rename = "devMode")]
    pub dev_mode: bool,
    pub methods: Vec<BillingMethods>,
    pub frequency: BillingKind,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub update_at: DateTime<Utc>,
    pub __v: i64,
    pub url: String,
    pub id: String,
    #[serde(rename = "customerId")]
    pub customer_id: Option<CustomerId>,
    pub customer: Option<Customer>,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CreateBillingProduct {
    #[serde(rename = "externalId")]
    pub external_id: String,
    pub name: String,
    pub quantity: i64,
    pub price: f64,
    pub description: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateBillingData {
    frequency: BillingKind,
    methods: Vec<BillingMethods>,
    products: Vec<CreateBillingProduct>,
    #[serde(rename = "returnUrl")]
    return_url: String,
    #[serde(rename = "completionUrl")]
    completion_url: String,
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
