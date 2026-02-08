use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Topic {
    pub id: String,
    #[serde(rename = "subjectId")]
    pub subject_id: String,
    pub title: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TopicNew {
    #[serde(rename = "subjectId")]
    pub subject_id: String,
    pub title: String,
}

impl Topic {
    pub fn new(topic_new: TopicNew) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            subject_id: topic_new.subject_id,
            title: topic_new.title,
        }
    }
}