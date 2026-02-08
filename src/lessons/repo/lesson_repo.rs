use crate::lessons::models::lesson::{Lesson, LessonNew};

#[async_trait::async_trait]
pub trait LessonRepository {
    async fn db_get_lesson(&self, id: &str) -> sqlx::Result<Lesson, sqlx::Error>;
    async fn db_get_lessons_by_topic(&self, topic_id: &str) -> sqlx::Result<Vec<Lesson>, sqlx::Error>;
    async fn db_create_lesson(&self, lesson_new: &LessonNew) -> sqlx::Result<Lesson, sqlx::Error>;
}