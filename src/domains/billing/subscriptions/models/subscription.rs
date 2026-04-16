use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SubscriptionStatus {
    Pending,
    Active,
    Canceled,
    Expired,
    PastDue,
}

impl SubscriptionStatus {
    pub fn to_string(&self) -> String {
        match self {
            SubscriptionStatus::Pending => "pending".to_string(),
            SubscriptionStatus::Active => "active".to_string(),
            SubscriptionStatus::Canceled => "canceled".to_string(),
            SubscriptionStatus::Expired => "expired".to_string(),
            SubscriptionStatus::PastDue => "past_due".to_string(),
        }
    }
}

impl From<String> for SubscriptionStatus {
    fn from(status: String) -> Self {
        match status.as_str() {
            "pending" => SubscriptionStatus::Pending,
            "active" => SubscriptionStatus::Active,
            "canceled" => SubscriptionStatus::Canceled,
            "expired" => SubscriptionStatus::Expired,
            "past_due" => SubscriptionStatus::PastDue,
            _ => SubscriptionStatus::Expired,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: String,
    pub student_id: String,
    pub plan_id: String,
    pub status: SubscriptionStatus,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub cancel_at_period_end: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionNew {
    pub student_id: String,
    pub plan_id: String,
    pub status: String,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub cancel_at_period_end: bool,
}

#[derive(FromRow, Debug, Clone)]
pub struct SubscriptionFromDB {
    pub id: String,
    pub student_id: String,
    pub plan_id: String,
    pub status: String,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub cancel_at_period_end: bool,
}

impl Subscription {
    pub fn new(new_sub: &SubscriptionNew) -> Subscription {
        Self {
            id: Uuid::now_v7().to_string(),
            student_id: new_sub.student_id.clone(),
            plan_id: new_sub.plan_id.clone(),
            status: SubscriptionStatus::from(new_sub.status.clone()),
            current_period_start: new_sub.current_period_start,
            current_period_end: new_sub.current_period_end,
            cancel_at_period_end: new_sub.cancel_at_period_end,
        }
    }

    pub fn from_db(s: SubscriptionFromDB) -> Subscription {
        Self {
            id: s.id,
            student_id: s.student_id,
            plan_id: s.plan_id,
            status: SubscriptionStatus::from(s.status),
            current_period_start: s.current_period_start,
            current_period_end: s.current_period_end,
            cancel_at_period_end: s.cancel_at_period_end,
        }
    }
}