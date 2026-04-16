use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub price: i64,
    pub duration: i32,
    pub is_active: bool,
    pub grade: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlanNew {
    pub name: String,
    pub price: i64,
    pub duration: i32,
    pub is_active: bool,
    pub grade: i32,
}

impl Plan {
    pub fn new(new_plan: &PlanNew) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            name: new_plan.name.clone(),
            price: new_plan.price,
            duration: new_plan.duration,
            is_active: new_plan.is_active,
            grade: new_plan.grade,
        }
    }
}