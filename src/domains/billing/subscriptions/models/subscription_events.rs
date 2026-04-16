use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionActivatedEvent {
    pub student_id: String,
    pub status: String,
}

pub struct SubscriptionExpiredEvent {
    pub subscription_id: String,
    pub student_id: String,
    pub expired_at: DateTime<Utc>,
}
