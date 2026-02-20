use crate::authentication::middleware::middleware::middleware;
use crate::payments::checkouts_state::CheckoutsState;
use crate::payments::models::checkout::Checkout;
use crate::payments::models::events::payment_notification::YocoPaymentNotification;
use crate::payments::models::yoco_checkout_request::YocoCheckoutRequest;
use crate::payments::models::yoco_checkout_response::YocoCheckoutResponse;
use crate::payments::service::payment_service::PaymentService;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, HttpResponse, post};
use chrono::{Datelike, Utc};
use std::io::Error;

#[post("")]
pub async fn create_yoco_checkout(
    state: Data<CheckoutsState>,
    req: HttpRequest,
    payload: Json<YocoCheckoutRequest>,
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} making payment", claims.sub);
            match PaymentService::make_payment(payload.into_inner()).await {
                Ok(yoco_result) => {
                    log::info!("yoco_result: {:?}", yoco_result);
                    match yoco_result.json::<YocoCheckoutResponse>().await {
                        Ok(yoco_response) => {
                            let now = Utc::now();
                            let month = now.month() as i32;
                            let year = now.year();
                            log::info!("Creating checkout on our servers");
                            let checkout = Checkout {
                                id: yoco_response.clone().id,
                                student_id: claims.sub,
                                amount: yoco_response.clone().amount,
                                status: yoco_response.clone().status,
                                month,
                                year,
                                created_at: Utc::now(),
                                updated_at: Utc::now(),
                            };

                            // create our checkout here
                            let _checkout = state
                                .repo
                                .create_checkout(checkout)
                                .await
                                .expect("Unable to create checkout");

                            // create payment immediately here, then update status after

                            Ok(HttpResponse::Ok().json(yoco_response))
                        }
                        Err(error) => {
                            log::error!("{:?}", error);
                            Ok(HttpResponse::Ok().json(error.to_string()))
                        }
                    }
                }
                Err(error) => {
                    log::error!("{:?}", error);
                    Ok(HttpResponse::NotFound().json(error.to_string()))
                }
            }
        }
        Err(error) => {
            log::error!("{:?}", error);
            Ok(HttpResponse::Unauthorized().json(error.to_string()))
        }
    }
}

// webhook
#[post("/webhooks/payment-notification")]
pub async fn payment_notification_webhook(
    req: HttpRequest,
    payload: Json<YocoPaymentNotification>,
) -> actix_web::Result<HttpResponse, std::io::Error> {
    log::info!("Receiving payment notification");
    match req
        .headers()
        .get("webhook-signature")
        .and_then(|t| t.to_str().ok())
    {
        Some(signature) => {
            log::info!("Payment notification: {:?}", payload.into_inner());
            Ok(HttpResponse::Ok().status(StatusCode::OK).json(""))
        }
        None => {
            log::info!("Webhook-signature is missing: Payment notification not received");
            Err(Error::other("Webhook-signature is missing"))
        }
    }
}
