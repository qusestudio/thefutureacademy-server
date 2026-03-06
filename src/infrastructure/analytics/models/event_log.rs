use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EventLog {
    pub id: String,
    pub user_id: String,
    pub event_type: String,
    pub created_at: DateTime<Utc>,
}
