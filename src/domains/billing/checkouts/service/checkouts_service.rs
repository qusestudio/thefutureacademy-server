use std::sync::Arc;
use crate::domains::billing::checkouts::models::checkout::CheckoutStatus;
use crate::infrastructure::event_bus::event_bus::{Event, EventBus};
use actix_web::web;
use tokio::sync::broadcast::Receiver;
use crate::domains::billing::checkouts::models::checkouts_events::CheckoutCompletedEvent;
use crate::domains::billing::checkouts::repo::checkout_repository::CheckoutRepository;

#[derive(Clone)]
pub struct CheckoutsService {
    pub repo: Arc<dyn CheckoutRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}

impl CheckoutsService {
    pub async fn checkout_events_handler(&self, mut receiver: Receiver<Event>) {
        log::info!("Checkouts Service listening for events...");
        while let Ok(event) = receiver.recv().await {
            match event {
                Event::PaymentCompleted(notification) => {
                    log::debug!("Checkout: PaymentCompleted notification received, updating checkout status...");
                    let _update_checkout_status = self
                        .repo
                        .update_checkout_status(
                            CheckoutStatus::Completed,
                            notification.checkout_id.as_str(),
                        )
                        .await;

                    let checkout_completed = CheckoutCompletedEvent{
                        checkout_id: notification.checkout_id
                    };

                    if let Err(e) = self
                        .event_bus
                        .send(Event::CheckoutCompleted(checkout_completed)) {
                        log::error!("Failed to send checkout completed: {}", e);
                    };
                },
                Event::HealthCheck(message) => {
                    log::info!("Checkouts Service: message received =>, \"{}\"", message.message);
                }
                _ => {}
            }
        }
    }
}
