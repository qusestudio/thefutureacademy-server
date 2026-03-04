use std::sync::Arc;
use actix_web::web;
use tokio::sync::broadcast::Receiver;
use crate::domains::billing::subscription::repo::subscription_repo::SubscriptionRepository;
use crate::infrastructure::event_bus::event_bus::{Event, EventBus};

pub struct SubscriptionsService {
    pub repo: Arc<dyn SubscriptionRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>
}

impl SubscriptionsService {
    pub async fn subscription_events_handler(&self, mut receiver: Receiver<Event>) {
        log::info!("Subscriptions Service listening for events...");
        while let Ok(event) = receiver.recv().await {
            match event {
                Event::HealthCheck(message) => {
                    log::info!("Subscriptions Service: message received =>, \"{}\"", message.message);
                }
                _ => {}
            }
        }
    }
}