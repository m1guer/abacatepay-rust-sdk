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
#[serde(rename_all = "camelCase")]
pub struct PixChargeData {
    pub amount: i64,
    pub status: String,
    pub dev_mode: bool,
    pub method: String,
    pub br_code: String,
    pub br_code_base64: String,
    pub platform_fee: i64,
    pub created_at: String,
    pub updated_at: String,
    pub expires_at: String,
    pub id: String,
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
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
        message: String,
        code: String,
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
        message: String,
        code: String,
    },
}
