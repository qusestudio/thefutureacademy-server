use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use crate::domains::billing::checkouts::models::checkouts_events::CheckoutCompletedEvent;
use crate::domains::billing::payments::models::payment_events::PaymentCompletedEvent;
use crate::domains::channel::events_channel_checker::EventMessage;

pub type EventBus = broadcast::Sender<Event>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    HealthCheck(EventMessage),
    PaymentCompleted(PaymentCompletedEvent),
    PaymentFailed,
    SubscriptionActivated,
    SubscriptionExpired,
    CheckoutCompleted(CheckoutCompletedEvent)
}
 
pub fn init_bus() -> broadcast::Sender<Event> {
    let (tx, _rx) = broadcast::channel(16);
    tx
}
