use std::sync::Arc;
use actix_web::web;
use crate::domains::users::instructors::repository::instructor_repo::InstructorRepository;
use crate::infrastructure::event_bus::event_bus::EventBus;

pub struct InstructorsService {
    pub repo: Arc<dyn InstructorRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}