use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub id: String,
    pub checkout_id: String,
    pub subscription_id: String,
    pub amount_received: i64,
    pub currency: String,
    /** Gateway (Yoco) Payment ID */
    pub transaction_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PaymentNew {
    pub checkout_id: String,
    pub subscription_id: String,
    pub amount_received: i64,
    pub currency: String,
    /** Gateway (Yoco) Payment ID */
    pub transaction_id: String,
    pub created_at: DateTime<Utc>,
}
