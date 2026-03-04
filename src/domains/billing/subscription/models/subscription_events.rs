use chrono::{DateTime, Utc};

pub struct SubscriptionActivatedEvent {
    pub subscription_id: String,
    pub student_id: String,
    pub activated_at: DateTime<Utc>,
}

pub struct SubscriptionExpiredEvent {
    pub subscription_id: String,
    pub student_id: String,
    pub expired_at: DateTime<Utc>,
}
