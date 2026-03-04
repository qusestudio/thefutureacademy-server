use std::sync::Arc;
use actix_web::web;
use crate::domains::enrollments::repo::enrollment_repo::EnrollmentRepository;
use crate::infrastructure::event_bus::event_bus::EventBus;

pub struct EnrollmentsService {
    pub repo: Arc<dyn EnrollmentRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}