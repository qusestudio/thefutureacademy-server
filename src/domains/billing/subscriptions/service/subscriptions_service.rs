use std::io::Error;
use std::sync::Arc;
use actix_web::web;
use tokio::sync::broadcast::Receiver;
use crate::domains::billing::subscriptions::models::subscription::{Subscription, SubscriptionNew};
use crate::domains::billing::subscriptions::models::subscription_events::SubscriptionActivatedEvent;
use crate::domains::billing::subscriptions::repo::subscription_repo::SubscriptionRepository;
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
                Event::CheckoutCompleted(message) => {
                    log::info!("Subscriptions Service: checkout.completed =>, \"{}\"", message.checkout_id);
                    let _update_subscription_status = self
                        .update_subscription_status("active".to_string(), message.student_id.clone())
                        .await;

                    let payload = SubscriptionActivatedEvent {
                        student_id: message.student_id.clone(),
                        status: "active".to_string()
                    };
                    
                    if let Err(e) = self
                        .event_bus
                        .send(Event::SubscriptionActivated(payload)) {
                        log::error!("Failed to send subscription.activated: {}", e);
                    };
                }
                _ => {}
            }
        }
    }
    
    pub async fn create_subscription(&self, ns: SubscriptionNew) -> Result<Subscription, Error> {
        let subscription = Subscription::new(&ns);
        match self.repo.create_subscription(subscription).await {
            Ok(subscription) => Ok(subscription),
            Err(e) => {
                Err(Error::other(e.to_string()))
            }
        }
    }
    
    pub async fn get_subscription_by_student(&self, id: String) -> Result<Subscription, Error> {
        match self.repo.get_subscription_by_student_id(id).await {
            Ok(subscription) => Ok(subscription),
            Err(e) => {
                Err(Error::other(e.to_string()))
            }
        }
    }

    pub async fn get_subscriptions_for_student(&self, id: String) -> Result<Vec<Subscription>, Error> {
        match self.repo.get_subscriptions_for_student(id).await {
            Ok(subscriptions) => Ok(subscriptions),
            Err(e) => {
                Err(Error::other(e.to_string()))
            }
        }
    }
    
    pub async fn update_subscription_status(&self, status: String, id: String) -> Result<bool, Error> {
        match self.repo.update_subscription_status(status, id).await {
            Ok(updated) => Ok(updated),
            Err(e) => {
                Err(Error::other(e.to_string()))
            }
        }
    }
    
}