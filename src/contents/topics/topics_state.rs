use std::sync::Arc;
use crate::contents::topics::repo::topic_repo::TopicRepository;

pub struct TopicsState {
    pub repo: Arc<dyn TopicRepository + Send + Sync>
}
