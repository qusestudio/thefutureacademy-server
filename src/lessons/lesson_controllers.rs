use actix_web::{get, post, web, HttpRequest, HttpResponse};
use actix_web::web::{Json, Path};
use crate::lessons::lessons_state::LessonsState;
use crate::lessons::models::lesson::LessonNew;
use crate::middleware::middleware::middleware;

#[get("/{id}")]
pub async fn get_lesson(state: web::Data<LessonsState>, req: HttpRequest, id: Path<String>) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} getting lesson with id {}", claims.sub ,&id.clone());
            match state.repo.db_get_lesson(id.into_inner().as_str()).await {
                Ok(lesson) => {
                    Ok(HttpResponse::Ok().json(Json(lesson)))
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

#[get("/{topic_id}/lessons")]
pub async fn get_lessons_by_topic(state: web::Data<LessonsState>, req: HttpRequest, topic_id: Path<String>) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} getting lessons by topic id {}", claims.sub ,&topic_id.clone());
            match state.repo.db_get_lessons_by_topic(topic_id.into_inner().as_str()).await {
                Ok(lesson) => {
                    Ok(HttpResponse::Ok().json(Json(lesson)))
                },
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

#[post("")]
pub async fn create_lesson(state: web::Data<LessonsState>, req: HttpRequest, lesson_new: Json<LessonNew>) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} creating a new lesson", claims.sub);
            match state.repo.db_create_lesson(&lesson_new.into_inner()).await {
                Ok(lesson) => {
                    Ok(HttpResponse::Ok().json(Json(lesson)))
                },
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