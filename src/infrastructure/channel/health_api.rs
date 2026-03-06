use actix_web::{post, web, HttpResponse};
use actix_web::web::Json;
use crate::configuration::state::AppState;
use crate::infrastructure::channel::events_channel_checker::{EventMessage, EventsChannelChecker};

#[post("/channel")]
pub async fn send_test_event(
    channel: web::Data<AppState>,
    payload: Json<EventMessage>
) -> actix_web::Result<HttpResponse> {
    log::debug!("{:?}", payload);
    // send test event to check health.
    channel.health_check_service.send_test_event(&payload.into_inner()).await;
    Ok(HttpResponse::Ok().finish())
}