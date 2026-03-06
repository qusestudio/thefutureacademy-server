use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all="camelCase")]
pub struct InstructorProfile {
    pub id: String,
    pub instructor_id: String, // Foreign Key

    pub first_name: String,
    pub last_name: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifications: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years_of_experience: Option<i32>,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct InstructorProfileNew {
    pub instructor_id: String,

    pub first_name: String,
    pub last_name: String,
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifications: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years_of_experience: Option<i32>,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructorProfileCreatedEvent {
    pub instructor_id: String,
}