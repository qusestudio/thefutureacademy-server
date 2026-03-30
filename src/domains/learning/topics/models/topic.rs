use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Topic {
    pub id: String,
    #[serde(rename = "subjectId")]
    pub subject_id: String,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TopicNew {
    #[serde(rename = "subjectId")]
    pub subject_id: String,
    pub title: String,
    pub description: String,
}

impl Topic {
    pub fn new(topic_new: TopicNew) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            subject_id: topic_new.subject_id,
            title: topic_new.title,
            description: topic_new.description,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopicViewedEvent {
    pub user_id: String,
    pub topic_title: String,
}

impl TopicViewedEvent {
    pub fn new(user_id: &str, topic_title: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            topic_title: topic_title.to_string(),
        }
    }
}