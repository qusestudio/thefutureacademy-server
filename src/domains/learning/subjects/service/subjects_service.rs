use std::sync::Arc;
use actix_web::web;
use crate::infrastructure::event_bus::event_bus::EventBus;
use crate::domains::learning::subjects::repo::subject_repo::SubjectRepository;

pub struct SubjectsService {
    pub repo: Arc<dyn SubjectRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}
