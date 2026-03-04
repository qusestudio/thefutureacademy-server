use std::sync::Arc;
use actix_web::web;
use crate::infrastructure::event_bus::event_bus::EventBus;
use crate::domains::learning::topics::repo::topic_repo::TopicRepository;

pub struct TopicsService {
    pub repo: Arc<dyn TopicRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}