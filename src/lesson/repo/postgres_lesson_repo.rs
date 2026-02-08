use sqlx::Error;
use crate::lesson::models::lesson::{Lesson, LessonNew};
use crate::lesson::repo::lesson_repo::LessonRepository;

pub struct PostgresLessonRepo {
    pub pg_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl LessonRepository for PostgresLessonRepo {
    async fn db_get_lesson(&self, id: &str) -> sqlx::Result<Lesson, Error> {
        todo!()
    }

    async fn db_get_lessons_by_topic(&self, topic_id: &str) -> sqlx::Result<Vec<Lesson>, Error> {
        todo!()
    }

    async fn db_create_lesson(&self, lesson_new: &LessonNew) -> sqlx::Result<Lesson, Error> {
        todo!()
    }
}