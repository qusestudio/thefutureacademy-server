use sqlx::Error;
use crate::domains::learning::topics::models::topic::{Topic, TopicNew};

#[async_trait::async_trait]
pub trait TopicRepository {
    async fn db_get_topic(&self, id: String) -> sqlx::Result<Topic, Error>;
    async fn db_get_topics_by_subject(&self, subject_id: &str) -> sqlx::Result<Vec<Topic>, Error>;
    async fn db_create_topic(&self, topic_new: TopicNew) -> sqlx::Result<Topic, Error>;
    async fn db_delete_topics(&self, ids: Vec<String>) -> sqlx::Result<u64, Error>;
}