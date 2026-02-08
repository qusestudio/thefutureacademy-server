use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lesson {
    pub id: String,
    #[serde(rename="videoId")]
    pub video_id: String,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LessonNew {
    #[serde(rename="videoId")]
    pub video_id: String,
    pub title: String,
    pub description: String,
}