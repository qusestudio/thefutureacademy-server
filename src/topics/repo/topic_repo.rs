use crate::topics::models::topic::{Topic, TopicNew};

#[async_trait::async_trait]
pub trait TopicRepository {
    async fn db_get_topic(&self, id: String) -> sqlx::Result<Topic, sqlx::Error>;
    async fn db_get_topics_by_subject(&self, subject_id: &str) -> sqlx::Result<Vec<Topic>, sqlx::Error>;
    async fn db_create_topic(&self, topic_new: TopicNew) -> sqlx::Result<Topic, sqlx::Error>;
}