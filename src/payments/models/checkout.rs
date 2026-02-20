use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename="camelCase")]
pub struct Checkout {
    pub id: String,
    pub student_id: String,
    pub amount: i64,
    pub status: String,
    pub month: i32,
    pub year: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}