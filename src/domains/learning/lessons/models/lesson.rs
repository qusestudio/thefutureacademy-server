use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct Lesson {
    pub id: String,
    #[serde(rename = "topicId")]
    pub topic_id: String,
    #[serde(rename="videoId")]
    pub video_id: String,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LessonNew {
    #[serde(rename = "topicId")]
    pub topic_id: String,
    #[serde(rename="videoId")]
    pub video_id: String,
    pub title: String,
    pub description: String,
}

impl Lesson {
    pub fn new(new: &LessonNew) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            topic_id: new.topic_id.clone(),
            video_id: new.video_id.clone(),
            title: new.title.clone(),
            description: new.description.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LessonCreatedEvent {
    pub user_id: String,
    pub lesson_title: String,
}

impl LessonCreatedEvent {
    pub fn new(user_id: &str, lesson_title: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            lesson_title: lesson_title.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LessonOpenedEvent {
    pub user_id: String,
    pub lesson_title: String,
}

impl LessonOpenedEvent {
    pub fn new(user_id: &str, lesson_title: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            lesson_title: lesson_title.to_string(),
        }
    }
}