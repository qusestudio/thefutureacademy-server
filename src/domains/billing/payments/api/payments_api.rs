use crate::infrastructure::middleware::middleware::middleware;
use crate::domains::billing::checkouts::models::checkout::{Checkout, CheckoutStatus};
use crate::domains::billing::payments::models::payment::Payment;
use crate::domains::billing::checkouts::models::yoco_checkout_request::YocoCheckoutRequest;
use crate::domains::billing::checkouts::models::yoco_checkout_response::YocoCheckoutResponse;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpRequest, HttpResponse, post, get};
use chrono::{Datelike, Utc};
use uuid::Uuid;
use crate::configuration::state::AppState;
use crate::domains::billing::payments::models::payment_events::{PaymentCompletedEvent, YocoPaymentNotification};
use crate::domains::billing::payments::service::payments_service::PaymentsService;
use crate::infrastructure::event_bus::event_bus::Event;

// #[post("")]
// pub async fn create_yoco_checkout(
//     c_state: Data<AppState>,
//     req: HttpRequest,
//     payload: Json<YocoCheckoutRequest>,
// ) -> actix_web::Result<HttpResponse, actix_web::Error> {
//     match middleware(req).await {
//         Ok(claims) => {
//             log::info!("User {} making payment", claims.sub);
//             match PaymentsService::make_payment(payload.into_inner()).await {
//                 Ok(yoco_result) => {
//                     log::info!("yoco_result: {:?}", yoco_result);
//                     match yoco_result.json::<YocoCheckoutResponse>().await {
//                         Ok(yoco_response) => {
//                             // Fetch Student's Subscription
//                             // TODO: FINISH THE PLAN MODULE
//
//                             log::info!("Creating checkout on our servers");
//                             let checkout = Checkout {
//                                 id: Uuid::now_v7().to_string(),
//                                 student_id: claims.sub,
//                                 // TODO: FINISH THE PLAN MODULE
//                                 plan_id: "".to_string(),
//                                 status: CheckoutStatus::Pending,
//                                 gateway_reference: yoco_response.id.clone(),
//                             };
//
//                             // create our checkout here
//                             let _checkout = c_state
//                                 .checkouts
//                                 .repo
//                                 .create_checkout(checkout)
//                                 .await
//                                 .expect("Unable to create checkout");
//
//                             Ok(HttpResponse::Ok().json(yoco_response))
//                         }
//                         Err(error) => {
//                             log::error!("{:?}", error);
//                             Ok(HttpResponse::Ok().json(error.to_string()))
//                         }
//                     }
//                 }
//                 Err(error) => {
//                     log::error!("{:?}", error);
//                     Ok(HttpResponse::NotFound().json(error.to_string()))
//                 }
//             }
//         }
//         Err(error) => {
//             log::error!("{:?}", error);
//             Ok(HttpResponse::Unauthorized().json(error.to_string()))
//         }
//     }
// }

#[get("/{student_id}/checkout")]
pub async fn get_checkout_by_student(
    c_state: Data<AppState>,
    req: HttpRequest,
    student_id: Path<String>,
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} fetching checkout info", claims.sub.clone());
            match c_state.checkouts.repo.get_checkout_by_student_id(student_id.as_str()).await {
                Ok(checkout) => {
                    Ok(HttpResponse::Ok().json(checkout))
                },
                Err(error) => {
                    log::error!("{:?}", error);
                    Ok(HttpResponse::Unauthorized().json(error.to_string()))
                }
            }
        },
        Err(error) => {
            log::error!("{:?}", error);
            Ok(HttpResponse::Unauthorized().json(error.to_string()))
        }
    }
}


// webhook
#[post("/webhooks/payment-notification")]
pub async fn payment_notification_webhook(
    service: Data<AppState>,
    req: HttpRequest,
    payload: actix_web::Result<Json<YocoPaymentNotification>>,
) -> actix_web::Result<HttpResponse> {
    log::error!("Receiving payment notification");

    // Handle deserialization errors
    let payload = match payload {
        Ok(p) => p,
        Err(e) => {
            log::error!("Failed to deserialize webhook payload: {}", e);
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Invalid payload: {}", e)
            })));
        }
    };

    // Check signature
    match req
        .headers()
        .get("webhook-signature")
        .and_then(|t| t.to_str().ok())
    {
        Some(signature) => {
            // take the payload and update the payment and checkout records accordingly
            log::info!("Payment notification: {:?}", payload.clone());

            match payload.clone().payload.metadata.unwrap().get("checkoutId") {
                // TODO: Something has to shift here a little. Since our checkout ID
                // TODO: now is a v7 uuid not the one from yoco.
                Some(checkout_id) => {

                    let payment = Payment {
                        id: Uuid::now_v7().to_string(),
                        checkout_id: checkout_id.clone(),
                        amount_received: payload.payload.amount,
                        currency: "ZAR".to_string(),
                        transaction_id: payload.payload.id.clone(),
                        created_at: Utc::now(),
                    };


                    let payment_completed = PaymentCompletedEvent{
                        checkout_id: checkout_id.clone(),
                    };
                    log::info!("Broadcasting payment event: {:?}", payment);
                    if let Err(e) = service
                        .payments
                        .event_bus
                        .send(Event::PaymentCompleted(payment_completed)) {
                        log::error!("Failed to send payment completed: {}", e);
                    };

                    log::info!("Recording payment in our servers: {:?}", payment);
                    match service.payments.repo.create_payment(payment).await {
                        Ok(payment) => match payment {
                            None => {
                                log::error!("Payment record not created or not returned!");
                                Ok(HttpResponse::NotFound().json(
                                    "Could not return payment record, might not be created at all.",
                                ))
                            },
                            Some(payment) => {
                                log::info!("Payment record created: {:?}", payment);

                                let payment_completed = PaymentCompletedEvent{
                                    checkout_id: payment.checkout_id,
                                };

                                if let Err(e) = service
                                    .payments
                                    .event_bus
                                    .send(Event::PaymentCompleted(payment_completed)) {
                                    log::error!("Failed to send payment completed: {}", e);
                                };
                                
                                Ok(HttpResponse::Ok().finish())
                            },
                        },
                        Err(error) => {
                            log::error!("Payment not returned for some reason. {:?}", error);
                            Ok(HttpResponse::NotFound().json(error.to_string()))
                        },
                    }
                }
                None => {
                    log::error!("Checkout Id not found, payment cannot be reconciled to any checkout!");
                    Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "error": "Checkout Id not found, payment cannot be reconciled!"
                    })))
                }
            }
        }
        None => {
            log::warn!("Webhook-signature is missing");
            Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Webhook-signature is missing"
            })))
        }
    }
}
