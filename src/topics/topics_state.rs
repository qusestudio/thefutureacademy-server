use std::sync::Arc;
use crate::topics::repo::topic_repo::TopicRepository;

pub struct TopicsState {
    pub repo: Arc<dyn TopicRepository + Send + Sync>
}
