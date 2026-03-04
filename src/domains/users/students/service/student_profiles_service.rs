use std::sync::Arc;
use actix_web::web;
use crate::domains::users::students::repository::student_profile_repo::StudentProfileRepository;
use crate::infrastructure::event_bus::event_bus::EventBus;

pub struct StudentProfilesService {
    pub repo: Arc<dyn StudentProfileRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}