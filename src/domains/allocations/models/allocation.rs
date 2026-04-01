use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Allocation {
    pub id: String,
    pub subject_id: String,
    pub instructor_id: String,
    pub created_at: DateTime<Utc>
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllocationNew {
    pub subject_id: String,
    pub instructor_id: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeachingAllocation {
    pub instructor_id: String,
    pub instructor_name: String,
    pub subject_id: String,
    pub subject_title: String,
    pub subject_image: String,
    pub grade: i32,
    pub term: i32,
}

impl Allocation {
    pub fn new(new_allocation: AllocationNew) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            subject_id: new_allocation.subject_id,
            instructor_id: new_allocation.instructor_id,
            created_at: Utc::now(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InstructorAllocatedEvent {
    pub instructor_id: String,
}

impl InstructorAllocatedEvent {
    pub fn new(instructor_id: &str) -> Self {
        Self {
            instructor_id: instructor_id.to_string(),
        }
    }
}