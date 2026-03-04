use std::sync::Arc;
use actix_web::web::Data;
use crate::domains::billing::checkouts::models::yoco_checkout_request::YocoCheckoutRequest;
use reqwest::header::HeaderMap;
use tokio::sync::broadcast::Receiver;
use crate::domains::billing::checkouts::models::checkout::CheckoutStatus;
use crate::domains::billing::checkouts::models::checkouts_events::CheckoutCompletedEvent;
use crate::domains::billing::payments::repo::payment_repo::PaymentRepository;
use crate::infrastructure::event_bus::event_bus::{Event, EventBus};

pub struct PaymentsService {
    pub repo: Arc<dyn PaymentRepository + Sync + Send>,
    pub event_bus: Data<EventBus>,
}

impl PaymentsService {
    // OUTBOUND
    pub async fn make_payment(
        checkout_request: YocoCheckoutRequest,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = "https://payments.yoco.com/api/checkouts";
        let token = std::env::var("PAYMENT_TOKEN_TEST").expect("PAYMENT_TOKEN not set");
        let token = format!("Bearer {}", token);
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Authorization", token.parse().unwrap());

        match reqwest::Client::new()
            .post(url)
            .headers(headers)
            .json(&checkout_request)
            .send()
            .await
        {
            Ok(response) => {
                log::info!("Response Status received: {}", response.status());
                Ok(response)
            },
            Err(error) => {
                log::error!("make_payment error: {}", error);
                Err(error)
            },
        }
    }

    pub async fn payment_events_handler(&self, mut receiver: Receiver<Event>) {
        log::info!("Payments Service listening for events...");
        while let Ok(event) = receiver.recv().await {
            match event {
                Event::HealthCheck(message) => {
                    log::info!("Payments Service: message received => , \"{}\"", message.message);
                }
                _ => {}
            }
        }
    }
}