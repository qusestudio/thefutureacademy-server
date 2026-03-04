use std::sync::Arc;
use actix_web::web;
use crate::domains::users::instructors::repository::instructor_profile_repository::InstructorProfileRepository;
use crate::infrastructure::event_bus::event_bus::EventBus;

pub struct InstructorProfilesService {
    pub repo: Arc<dyn InstructorProfileRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}