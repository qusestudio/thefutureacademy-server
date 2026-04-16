use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename = "camelCase")]
pub struct Checkout {
    pub id: String,
    pub student_id: String,
    pub plan_id: String,
    pub status: CheckoutStatus,
    /** Payment Gateway (YOCO) Id */
    pub gateway_reference: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename = "camelCase")]
pub struct CheckoutFromDB {
    pub id: String,
    pub student_id: String,
    pub plan_id: String,
    pub status: String,
    /** Payment Gateway (YOCO) Id */
    pub gateway_reference: String,
}

pub struct CheckoutNew {
    pub id: String,
    pub student_id: String,
    pub plan_id: String,
    pub status: CheckoutStatus,
    pub gateway_reference: String,
}

impl Checkout {
    pub fn new(new_ckt: &CheckoutNew) -> Self {
        Self {
            id: new_ckt.id.clone(),
            student_id: new_ckt.student_id.clone(),
            plan_id: new_ckt.plan_id.clone(),
            status: new_ckt.status.clone(),
            gateway_reference: new_ckt.gateway_reference.clone(),
        }
    }

    pub fn from_checkout_db(db_ckt: CheckoutFromDB) -> Self {
        Self {
            id: db_ckt.id.clone(),
            student_id: db_ckt.student_id.clone(),
            plan_id: db_ckt.plan_id.clone(),
            status: db_ckt.status.clone().into(),
            gateway_reference: db_ckt.gateway_reference.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckoutStatus {
    Pending,
    Completed,
    Abandoned,
}

impl CheckoutStatus {
    pub fn to_string(self) -> String {
        match self {
            CheckoutStatus::Pending => "pending".to_string(),
            CheckoutStatus::Completed => "completed".to_string(),
            CheckoutStatus::Abandoned => "abandoned".to_string(),
        }
    }
}

impl Into<CheckoutStatus> for String {
    fn into(self) -> CheckoutStatus {
        match self.as_str() {
            "pending" => CheckoutStatus::Pending,
            "completed" => CheckoutStatus::Completed,
            "abandoned" => CheckoutStatus::Abandoned,
            _ => CheckoutStatus::Pending,
        }
    }
}
