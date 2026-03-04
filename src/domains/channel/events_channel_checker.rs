use actix_web::web;
use serde::{Deserialize, Serialize};
use crate::infrastructure::event_bus::event_bus::{Event, EventBus};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventMessage {
    pub message: String,
}

pub struct EventsChannelChecker {
    pub event_bus: web::Data<EventBus>,
}

impl EventsChannelChecker {
    pub async fn send_test_event(&self, event: &EventMessage) {
        if let Err(e) = self.event_bus.send(Event::HealthCheck(event.clone())) {
            log::error!("Failed to send test event: {}", e);
        };
    }
}