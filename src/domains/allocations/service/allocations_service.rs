use std::sync::Arc;
use actix_web::web::Data;
use crate::domains::allocations::repo::allocation_repo::AllocationRepository;
use crate::infrastructure::event_bus::event_bus::EventBus;

pub struct AllocationsService {
    pub repo: Arc<dyn AllocationRepository + Send + Sync>,
    pub event_bus: Data<EventBus>,
}