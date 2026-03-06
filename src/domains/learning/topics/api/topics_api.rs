use crate::domains::learning::topics::models::topic::TopicNew;
use crate::infrastructure::middleware::middleware::middleware;
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, get, post, web, delete};
use serde_json::json;
use crate::configuration::state::AppState;

// /topics/{topic_id}
#[get("/{topic_id}")]
pub async fn get_topic(
    state: web::Data<AppState>,
    req: HttpRequest,
    topic_id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!(
                "User {} getting Topic with id {}",
                claims.sub,
                &topic_id.clone()
            );

            match state.topics.get_topic(&claims.sub, topic_id.into_inner().as_str()).await {
                Ok(topic) => Ok(HttpResponse::Ok().json(Json(topic))),
                Err(_) => Ok(HttpResponse::NotFound().json("Topic not found")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

// /subjects/{subject_id}/topics
#[get("/{subject_id}/topics")]
pub async fn get_topics_by_subject(
    state: web::Data<AppState>,
    req: HttpRequest,
    subject_id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!(
                "User {} getting Topics with subject id {}",
                claims.sub,
                &subject_id.clone()
            );

            match state
                .topics
                .repo
                .db_get_topics_by_subject(subject_id.into_inner().as_str())
                .await
            {
                Ok(topics) => Ok(HttpResponse::Ok().json(Json(topics))),
                Err(_) => Ok(HttpResponse::NotFound().json("Topics not found")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

// /topics
#[post("")]
pub async fn create_topic(
    state: web::Data<AppState>,
    req: HttpRequest,
    topic_new: Json<TopicNew>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} creating a new topic", claims.sub);
            match state.topics.repo.db_create_topic(topic_new.into_inner()).await {
                Ok(topic) => Ok(HttpResponse::Ok().json(Json(topic))),
                Err(e) => Ok(HttpResponse::NotFound().json(e.to_string())),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[delete("")]
pub async fn delete_topics(
    req: HttpRequest,
    state: web::Data<AppState>,
    payload: Json<Vec<String>>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => match claims.custom_role.as_str() {
            "admin" => {
                log::info!("Admin deleting topics");
                match state
                    .topics
                    .repo
                    .db_delete_topics(payload.into_inner())
                    .await
                {
                    Ok(rows_affected) => Ok(HttpResponse::Ok().json(json!({
                        "rows_affected": rows_affected
                    }))),
                    Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
                }
            }
            _ => Ok(HttpResponse::Forbidden().json("Not allowed")),
        },
        Err(e) => Ok(HttpResponse::Unauthorized().json(format!("{}", e))),
    }
}