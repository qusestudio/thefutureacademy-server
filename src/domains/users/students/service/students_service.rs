use std::sync::Arc;
use actix_web::web;
use crate::domains::users::students::repository::student_repo::StudentRepository;
use crate::infrastructure::event_bus::event_bus::EventBus;

pub struct StudentsService {
    pub repo: Arc<dyn StudentRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}

