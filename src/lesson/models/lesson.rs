use serde::Serialize;

#[derive(Serialize)]
pub struct Lesson {
    pub id: String,
    #[serde(rename="videoId")]
    pub video_id: String,
    pub title: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct LessonNew {
    #[serde(rename="videoId")]
    pub video_id: String,
    pub title: String,
    pub description: String,
}