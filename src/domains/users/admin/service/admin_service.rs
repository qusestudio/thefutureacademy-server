use std::sync::Arc;
use actix_web::web;
use crate::domains::users::admin::repo::admin_repo::AdminRepository;
use crate::infrastructure::event_bus::event_bus::EventBus;

pub struct AdminsService {
    pub repo: Arc<dyn AdminRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>
}