use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::billing::CustomerMetadata;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PixStatus {
    PENDING,
    EXPIRED,
    CANCELLED,
    PAID,
    REFUNDED,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PixChargeData {
    pub id: String,
    pub amount: f64,
    pub status: PixStatus,
    pub dev_mode: bool,
    pub br_code: String,
    pub br_code_base64: String,
    pub platform_fee: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckPixStatusData {
    pub status: PixStatus,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePixChargeData {
    pub amount: f64,
    pub expires_in: Option<u64>,
    pub description: Option<String>,
    pub customer: Option<CustomerMetadata>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CheckPixStatusResponse {
    Success {
        error: Option<()>,
        data: CheckPixStatusData,
    },
    Error {
        error: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PixChargeResponse {
    Success {
        error: Option<()>,
        data: PixChargeData,
    },
    Error {
        error: String,
    },
}
