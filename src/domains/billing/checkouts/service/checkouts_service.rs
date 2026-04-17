use std::error::Error;
use std::sync::Arc;
use crate::domains::billing::checkouts::models::checkout::{Checkout, CheckoutNew, CheckoutStatus};
use crate::infrastructure::event_bus::event_bus::{Event, EventBus};
use actix_web::web;
use reqwest::header::HeaderMap;
use tokio::sync::broadcast::Receiver;
use crate::domains::billing::checkouts::models::checkouts_events::CheckoutCompletedEvent;
use crate::domains::billing::checkouts::models::yoco_checkout_request::YocoCheckoutRequest;
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
                    log::info!("Checkout: PaymentCompleted notification received, updating checkout status for checkout_id: {}...", notification.checkout_id);
                    let _update_checkout_status = self
                        .repo
                        .update_checkout_status(
                            CheckoutStatus::Completed,
                            notification.checkout_id.as_str(),
                        )
                        .await;

                    let checkout = self.repo.get_checkout(notification.checkout_id.as_str()).await.unwrap();

                    let checkout_completed = CheckoutCompletedEvent{
                        student_id: checkout.student_id,
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
    pub async fn initiate_yoco_checkout(
        checkout_request: YocoCheckoutRequest,
    ) -> Result<reqwest::Response, Box<dyn Error>> { // Changed to Box<dyn Error> to allow custom error messages
        let url = std::env::var("PAYMENT_GATEWAY_URL")
            .unwrap_or_else(|_| "https://payments.yoco.com/api/checkouts".to_string());

        // Tip: Use a single variable name like "PAYMENT_TOKEN" and just swap the actual
        // key in your .env file rather than hardcoding "TEST" or "LIVE" in the Rust code.
        let token = std::env::var("PAYMENT_TOKEN").expect("PAYMENT_TOKEN not set");
        let auth_value = format!("Bearer {}", token);

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("Authorization", auth_value.parse()?);

        // Send the request and catch pure network/connection errors early
        let response = reqwest::Client::new()
            .post(url)
            .headers(headers)
            .json(&checkout_request)
            .send()
            .await?;

        // 🚨 The Fix: Check if Yoco rejected the payload (e.g., 400 Bad Request)
        if !response.status().is_success() {
            let status = response.status();

            // Read the body to see exactly why the Live API rejected it
            let error_body = response.text().await?;
            log::error!("Yoco API Error! Status: {}, Body: {}", status, error_body);

            // Return an explicit error so the caller doesn't try to parse a missing `id`
            return Err(format!("Yoco API rejected the request. Status: {}, Details: {}", status, error_body).into());
        }

        // If we reach here, it is a 2xx success!
        log::info!("Response Status received: {}", response.status());
        Ok(response)
    }

    pub async fn create_checkout(&self, nckt: CheckoutNew) -> Result<Checkout, std::io::Error> {
        let ckt = Checkout::new(&nckt);
        match self.repo.create_checkout(ckt).await {
            Ok(checkout) => Ok(checkout),
            Err(error) => Err(std::io::Error::new(std::io::ErrorKind::Other, error))
        }
    }

    pub async fn get_checkout_by_student(&self, student_id: String) -> Result<Checkout, std::io::Error> {
        match self.repo.get_checkout_by_student_id(&student_id).await {
            Ok(checkout) => Ok(checkout),
            Err(error) => Err(std::io::Error::new(std::io::ErrorKind::Other, error))
        }
    }
}
