use actix_web::{get, post, web, HttpRequest, HttpResponse};
use actix_web::web::{Json, Path};
use crate::configuration::state::AppState;
use crate::domains::billing::subscriptions::models::subscription::{SubscriptionNew};
use crate::infrastructure::middleware::middleware::middleware;

#[get("/{student_id}/subscription")]
pub async fn get_subscription_by_student_id(
    state: web::Data<AppState>,
    req: HttpRequest,
    student_id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("Student {} requesting subscription", claims.sub);
            match state.subscriptions.get_subscription_by_student(student_id.into_inner()).await {
                Ok(subscription) => {
                    Ok(HttpResponse::Ok().json(subscription))
                },
                Err(error) => {
                    log::error!("{:?}", error);
                    Ok(HttpResponse::NotFound().json(error.to_string()))
                }
            }
        }
        Err(error) => Ok(
            HttpResponse::Unauthorized().json(error.to_string())
        )
    }
}

#[post("")]
pub async fn create_subscription(
    state: web::Data<AppState>,
    req: HttpRequest,
    payload: Json<SubscriptionNew>
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("Student {} requesting subscription", claims.sub);
            match state.subscriptions.create_subscription(payload.into_inner()).await {
                Ok(subscription) => {
                    Ok(HttpResponse::Ok().json(subscription))
                }
                Err(error) => {
                    log::error!("Error: creating subscription failed. {:?}", error);
                    Ok(HttpResponse::InternalServerError().json(error.to_string()))
                }
            }
        }
        Err(error) => Ok(HttpResponse::Unauthorized().json(error.to_string()))
    }
}