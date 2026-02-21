use crate::authentication::middleware::middleware::middleware;
use crate::payments::checkouts_state::CheckoutsState;
use crate::payments::models::checkout::Checkout;
use crate::payments::models::events::payment_notification::YocoPaymentNotification;
use crate::payments::models::payment::Payment;
use crate::payments::models::yoco_checkout_request::YocoCheckoutRequest;
use crate::payments::models::yoco_checkout_response::YocoCheckoutResponse;
use crate::payments::payments_state::PaymentsState;
use crate::payments::service::payment_service::PaymentService;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpRequest, HttpResponse, post, get};
use chrono::{Datelike, Utc};

#[post("")]
pub async fn create_yoco_checkout(
    c_state: Data<CheckoutsState>,
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
                            let _checkout = c_state
                                .repo
                                .create_checkout(checkout)
                                .await
                                .expect("Unable to create checkout");

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

#[get("/{student_id}/checkout")]
pub async fn get_checkout_by_student(
    c_state: Data<CheckoutsState>,
    req: HttpRequest,
    student_id: Path<String>,
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} fetching checkout info", claims.sub.clone());
            match c_state.repo.get_checkout(student_id.as_str()).await {
                Ok(checkout_result) => {
                    match checkout_result {
                        Some(checkout) => {
                            Ok(HttpResponse::Ok().json(checkout))
                        },
                        None => Ok(HttpResponse::NotFound().json("Checkout not found"))
                    }
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
    p_state: Data<PaymentsState>,
    c_state: Data<CheckoutsState>,
    req: HttpRequest,
    payload: actix_web::Result<Json<YocoPaymentNotification>>,
) -> actix_web::Result<HttpResponse> {
    log::info!("Receiving payment notification");

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
                Some(checkout_id) => {

                    let payment = Payment {
                        payment_id: payload.payload.id.clone(),
                        checkout_id: checkout_id.clone(),
                        status: payload.payload.status.clone(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    };

                    match p_state.repo.create_payment(payment).await {
                        Ok(payment) => match payment {
                            None => Ok(HttpResponse::NotFound().json(
                                "Could not return payment record, might not be created at all.",
                            )),
                            Some(payment) => {
                                // here the payment was successfully recorded.
                                let checkout_status: String;
                                match payment.status.as_str() {
                                    "succeeded" => {
                                        checkout_status = "Completed".to_string();
                                    },
                                    _ => {
                                        checkout_status = "Pending".to_string();
                                    }
                                }

                                let update_checkout_status = c_state
                                    .repo
                                    .update_checkout_status(checkout_status.as_str(), payment.checkout_id.as_str())
                                    .await;

                                log::info!("Update checkout status: {:?}", update_checkout_status);

                                Ok(HttpResponse::Ok().json(payment))
                            },
                        },
                        Err(error) => Ok(HttpResponse::NotFound().json(error.to_string())),
                    }
                }
                None => {
                    log::error!(
                        "Checkout Id not found, payment cannot be reconciled to any checkout!"
                    );
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
