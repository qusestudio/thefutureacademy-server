use std::error::Error;
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
    ) -> Result<reqwest::Response, Box<dyn Error>> { // Changed to Box<dyn Error> to allow custom error messages
        let url = std::env::var("PAYMENT_GATEWAY_URL")
            .unwrap_or_else(|_| "https://payments.yoco.com/api/checkouts".to_string());

        // Tip: Use a single variable name like "PAYMENT_TOKEN" and just swap the actual
        // key in your .env file rather than hardcoding "TEST" or "LIVE" in the Rust code.
        let token = std::env::var("PAYMENT_TOKEN").expect("PAYMENT_TOKEN not set");
        let auth_value = format!("Bearer {}", token);

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Authorization", auth_value.parse().unwrap());

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

    // pub async fn make_payment(
    //     checkout_request: YocoCheckoutRequest,
    // ) -> Result<reqwest::Response, reqwest::Error> {
    //     let url = "https://payments.yoco.com/api/checkouts";
    //     let token = std::env::var("PAYMENT_TOKEN_TEST").expect("PAYMENT_TOKEN not set");
    //     let token = format!("Bearer {}", token);
    //     let mut headers: HeaderMap = HeaderMap::new();
    //     headers.insert("Content-Type", "application/json".parse().unwrap());
    //     headers.insert("Authorization", token.parse().unwrap());
    //
    //     match reqwest::Client::new()
    //         .post(url)
    //         .headers(headers)
    //         .json(&checkout_request)
    //         .send()
    //         .await
    //     {
    //         Ok(response) => {
    //             log::info!("Response Status received: {}", response.status());
    //             Ok(response)
    //         },
    //         Err(error) => {
    //             log::error!("make_payment error: {}", error);
    //             Err(error)
    //         },
    //     }
    // }

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