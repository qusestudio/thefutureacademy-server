use actix_web::{get, post, web, HttpRequest, HttpResponse};
use actix_web::web::{Json, Path};
use crate::middleware::middleware::middleware;
use crate::topics::models::topic::TopicNew;
use crate::topics::topics_state::TopicsState;

// /topics/{topic_id}
#[get("/{topic_id}")]
pub async fn get_topic(state: web::Data<TopicsState>, req: HttpRequest, topic_id: Path<String>) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} getting Topic with id {}", claims.sub ,&topic_id.clone());

            match state.repo.db_get_topic(topic_id.into_inner()).await {
                Ok(topic) => {
                    Ok(HttpResponse::Ok().json(Json(topic)))
                }
                Err(_) => {
                    Ok(HttpResponse::NotFound().json("Topic not found"))
                }
            }
        }
        Err(e) => {
            Ok(HttpResponse::Unauthorized().json(e.to_string()))
        }
    }
}

// /subjects/{subject_id}/topics
#[get("/{subject_id}/topics")]
pub async fn get_topics_by_subject(state: web::Data<TopicsState>, req: HttpRequest, subject_id: Path<String>) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} getting Topics with subject id {}", claims.sub ,&subject_id.clone());

            match state.repo.db_get_topics_by_subject(subject_id.into_inner().as_str()).await {
                Ok(topics) => {
                    Ok(HttpResponse::Ok().json(Json(topics)))
                }
                Err(_) => {
                    Ok(HttpResponse::NotFound().json("Topics not found"))
                }
            }
        }
        Err(e) => {
            Ok(HttpResponse::Unauthorized().json(e.to_string()))
        }
    }
}

// /topics
#[post("")]
pub async fn create_topic(state: web::Data<TopicsState>, req: HttpRequest, topic_new: Json<TopicNew>) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} creating a new topic", claims.sub);

            match state.repo.db_create_topic(topic_new.into_inner()).await {
                Ok(topic) => {
                    Ok(HttpResponse::Ok().json(Json(topic)))
                }
                Err(e) => {
                    Ok(HttpResponse::NotFound().json(e.to_string()))
                }
            }
        }
        Err(e) => {
            Ok(HttpResponse::Unauthorized().json(e.to_string()))
        }
    }
}
