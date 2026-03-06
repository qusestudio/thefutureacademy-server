use actix_web::{get, post, web,  HttpRequest, HttpResponse};
use actix_web::web::{Json, Path};
use crate::configuration::state::AppState;
use crate::domains::learning::lessons::models::lesson::LessonNew;
use crate::infrastructure::middleware::middleware::middleware;

#[get("/{id}")]
pub async fn get_lesson(state: web::Data<AppState>, req: HttpRequest, id: Path<String>) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} getting lesson with id {}", &claims.sub ,&id.clone());
            match state.lessons.get_lesson(&claims.sub, id.into_inner().as_str()).await {
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
pub async fn get_lessons_by_topic(state: web::Data<AppState>, req: HttpRequest, topic_id: Path<String>) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} getting lessons by topic id {}", claims.sub ,&topic_id.clone());
            match state.lessons.get_lessons_by_topic_id(topic_id.into_inner().as_str()).await {
                Ok(lessons) => {
                    Ok(HttpResponse::Ok().json(Json(lessons)))
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
pub async fn create_lesson(state: web::Data<AppState>, req: HttpRequest, lesson_new: Json<LessonNew>) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("User {} creating a new lesson", claims.sub);
            match state.lessons.create_lesson( &claims.sub ,&lesson_new.into_inner()).await {
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