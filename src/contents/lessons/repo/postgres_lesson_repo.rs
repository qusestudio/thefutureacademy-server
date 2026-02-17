use crate::contents::lessons::models::lesson::{Lesson, LessonNew};
use crate::contents::lessons::repo::lesson_repo::LessonRepository;

pub struct PostgresLessonRepo {
    pub pg_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl LessonRepository for PostgresLessonRepo {
    async fn db_get_lesson(&self, id: &str) -> sqlx::Result<Lesson, sqlx::Error> {
        let lesson = sqlx::query_as("select * from lessons where id = $1")
            .bind(&id)
            .fetch_one(&self.pg_pool)
            .await?;

        Ok(lesson)
    }

    async fn db_get_lessons_by_topic(
        &self,
        topic_id: &str,
    ) -> sqlx::Result<Vec<Lesson>, sqlx::Error> {
        let lessons = sqlx::query_as("select * from lessons where topic_id = $1")
            .bind(&topic_id)
            .fetch_all(&self.pg_pool)
            .await?;

        Ok(lessons)
    }

    async fn db_create_lesson(&self, lesson_new: &LessonNew) -> sqlx::Result<Lesson, sqlx::Error> {
        let lesson = Lesson::new(lesson_new);
        let lesson = sqlx::query_as("
                    INSERT INTO lessons (id, topic_id, video_id, title, description) 
                    VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(&lesson.id)
            .bind(&lesson.topic_id)
            .bind(&lesson.video_id)
            .bind(&lesson.title)
            .bind(&lesson.description)
            .fetch_one(&self.pg_pool)
            .await?;

        Ok(lesson)
    }
}
