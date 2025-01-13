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
    Pix,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum BillingKind {
    OneTime,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub fee: i64,
    pub return_url: String,
    pub completion_url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub product_id: String,
    pub quantity: i64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomerMetadata {
    pub name: String,
    pub cellphone: String,
    pub tax_id: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomerId {
    pub metadata: CustomerMetadata,
    pub _id: String,
    pub public_id: String,
    pub store_id: String,
    pub dev_mode: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub __v: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Customer {
    pub _id: String,
    pub metadata: CustomerMetadata,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Billing {
    pub metadata: Metadata,
    #[serde(rename = "pId")]
    pub _id: String,
    pub public_id: String,
    pub products: Vec<Product>,
    pub amount: i64,
    pub status: BillingStatus,
    pub dev_mode: bool,
    pub methods: Vec<BillingMethods>,
    pub frequency: BillingKind,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
    pub __v: i64,
    pub url: String,
    pub id: String,
    pub customer_id: Option<CustomerId>,
    pub customer: Option<Customer>,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateBillingProduct {
    pub external_id: String,
    pub name: String,
    pub quantity: i64,
    pub price: f64,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateBillingData {
    pub frequency: BillingKind,
    pub methods: Vec<BillingMethods>,
    pub products: Vec<CreateBillingProduct>,
    pub return_url: String,
    pub completion_url: String,
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
