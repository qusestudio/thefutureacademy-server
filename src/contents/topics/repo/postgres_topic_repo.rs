use crate::contents::topics::models::topic::{Topic, TopicNew};
use crate::contents::topics::repo::topic_repo::TopicRepository;
use sqlx::Error;

pub struct PostgresTopicRepo {
    pub pg_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl TopicRepository for PostgresTopicRepo {
    async fn db_get_topic(&self, id: String) -> sqlx::Result<Topic, Error> {
        let topic = sqlx::query_as("SELECT * FROM topics WHERE id = $1")
            .bind(&id)
            .fetch_one(&self.pg_pool)
            .await?;

        Ok(topic)
    }

    async fn db_get_topics_by_subject(&self, subject_id: &str) -> sqlx::Result<Vec<Topic>, Error> {
        let topics = sqlx::query_as("SELECT * FROM topics WHERE subject_id = $1")
            .bind(subject_id)
            .fetch_all(&self.pg_pool)
            .await?;

        Ok(topics)
    }

    async fn db_create_topic(&self, topic_new: TopicNew) -> sqlx::Result<Topic, Error> {
        let topic = Topic::new(topic_new);
        let topic = sqlx::query_as(
            "INSERT INTO topics (id, subject_id, title ) VALUES($1, $2, $3) RETURNING *",
        )
        .bind(&topic.id)
        .bind(&topic.subject_id)
        .bind(&topic.title)
        .fetch_one(&self.pg_pool)
        .await?;

        Ok(topic)
    }
}
