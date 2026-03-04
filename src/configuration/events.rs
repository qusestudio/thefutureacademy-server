use std::sync::Arc;
use actix_web::web::Data;
use tokio::sync::broadcast::Sender;
use crate::configuration::state::AppState;
use crate::infrastructure::event_bus::event_bus::Event;

pub async fn event_handlers_init(tx: Sender<Event>, state: Data<AppState>) {
    // --- SPAWN CHECKOUT SUBSCRIBER ---
    let checkout_service = Arc::new(state.checkouts.clone());
    let rx_checkout = tx.subscribe();
    let checkout_clone = Arc::clone(&checkout_service);
    tokio::spawn(async move {
        checkout_clone.checkout_events_handler(rx_checkout).await;
    });

    // --- SPAWN PAYMENT SUBSCRIBER ---
    let payment_service = Arc::new(state.payments.clone());
    let rx_payment = tx.subscribe();
    let payment_clone = Arc::clone(&payment_service);
    tokio::spawn(async move {
        payment_clone.payment_events_handler(rx_payment).await;
    });

    // --- SPAWN SUBSCRIPTION SUBSCRIBER ---
    let subscription_service = Arc::new(state.subscriptions.clone());
    let rx_subscription = tx.subscribe();
    let subscription_clone = Arc::clone(&subscription_service);
    tokio::spawn(async move {
        subscription_clone.subscription_events_handler(rx_subscription).await;
    });
}