use crate::domains::learning::topics::models::topic::{Topic, TopicNew};
use crate::domains::learning::topics::repo::topic_repo::TopicRepository;
use sqlx::Error;

pub struct PostgresTopicRepo {
    pub pg_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl TopicRepository for PostgresTopicRepo {
    async fn db_get_topic(&self, topic_id: &str) -> sqlx::Result<Option<Topic>, Error> {
        let topic = sqlx::query_as("SELECT * FROM topics WHERE id = $1")
            .bind(&topic_id)
            .fetch_optional(&self.pg_pool)
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
            "INSERT INTO topics (id, subject_id, title, description ) VALUES($1, $2, $3, $4) RETURNING *",
        )
        .bind(&topic.id)
        .bind(&topic.subject_id)
        .bind(&topic.title)
        .bind(&topic.description)
        .fetch_one(&self.pg_pool)
        .await?;

        Ok(topic)
    }

    async fn db_delete_topics(&self, ids: Vec<String>) -> sqlx::Result<u64, Error> {
        let result = sqlx::query("DELETE FROM topics WHERE id = ANY($1)")
            .bind(&ids)
            .execute(&self.pg_pool)
            .await?;

        Ok(result.rows_affected())
    }
}
