use std::sync::Arc;
use actix_web::web;
use crate::infrastructure::event_bus::event_bus::EventBus;
use crate::domains::learning::lessons::repo::lesson_repo::LessonRepository;

pub struct LessonsService {
    pub repo: Arc<dyn LessonRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}