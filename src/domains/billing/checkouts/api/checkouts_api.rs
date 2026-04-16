use actix_web::{post, HttpRequest, HttpResponse};
use actix_web::web::{Data, Json};
use crate::configuration::state::AppState;
use crate::domains::billing::checkouts::models::checkout::{CheckoutNew, CheckoutStatus};
use crate::domains::billing::checkouts::models::yoco_checkout_request::YocoCheckoutRequest;
use crate::domains::billing::checkouts::models::yoco_checkout_response::YocoCheckoutResponse;
use crate::domains::billing::checkouts::service::checkouts_service::CheckoutsService;
use crate::infrastructure::middleware::middleware::middleware;

#[post("")]
pub async fn create_yoco_checkout(
    state: Data<AppState>,
    req: HttpRequest,
    payload: Json<YocoCheckoutRequest>,
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("Student {} initiating checkout", claims.sub);
            match CheckoutsService::initiate_yoco_checkout(payload.into_inner()).await {
                Ok(yoco_result) => {
                    log::info!("yoco_result: {:?}", yoco_result);
                    match yoco_result.json::<YocoCheckoutResponse>().await {
                        Ok(yoco_response) => {
                            // Fetch Student's Subscription
                            // TODO: FINISH THE PLAN MODULE
                            match state.subscriptions.get_subscription_by_student(claims.sub.clone()).await {
                                Ok(sub) => {
                                    log::info!("Creating checkout on our servers");
                                    // TODO: send this from the client
                                    let checkout_new = CheckoutNew {
                                        student_id: claims.sub,
                                        plan_id: sub.plan_id.clone(),
                                        status: CheckoutStatus::Pending,
                                        gateway_reference: yoco_response.id.clone(),
                                    };

                                    // create our checkout here
                                    let _checkout = state
                                        .checkouts
                                        .create_checkout(checkout_new)
                                        .await
                                        .expect("Unable to create checkout");
                                }
                                Err(error) => {
                                    log::error!("Could not find subscription: {:?}", error);
                                    return Ok(HttpResponse::Ok().json(error.to_string()));
                                }
                            }
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