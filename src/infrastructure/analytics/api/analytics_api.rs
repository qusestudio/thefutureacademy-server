use actix_web::{get, HttpRequest, HttpResponse};
use actix_web::web::Data;
use crate::configuration::state::AppState;
use crate::infrastructure::middleware::middleware::middleware;

#[get("/no-of-students")]
pub async fn get_no_of_students(
    req: HttpRequest,
    state: Data<AppState>
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            match claims.custom_role.as_str() {
                "admin" => {
                    match state.analytics.students_registered().await {
                        Ok(no_of_students) => Ok(HttpResponse::Ok().json(no_of_students)),
                        Err(e) => Err(actix_web::error::ErrorInternalServerError(e.to_string()))
                    }
                }
                _ => Ok(HttpResponse::Forbidden().json("Access denied."))
            }
        },
        Err(e) => {
            Ok(HttpResponse::Unauthorized().json(e.to_string()))
        }
    }
}

#[get("/no-of-instructors")]
pub async fn get_no_of_instructors(
    req: HttpRequest,
    state: Data<AppState>
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            match claims.custom_role.as_str() {
                "admin" => {
                    match state.analytics.instructors_registered().await {
                        Ok(no_of_instructors) => Ok(HttpResponse::Ok().json(no_of_instructors)),
                        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string()))
                    }
                }
                _ => Ok(HttpResponse::Forbidden().json("Access denied."))
            }
        }
        Err(e) => {
            Ok(HttpResponse::Unauthorized().json(e.to_string()))
        }
    }
}

#[get("no-of-enrolled-subjects")]
pub async fn get_no_of_enrollments(
    req: HttpRequest,
    state: Data<AppState>
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            match claims.custom_role.as_str() {
                "admin" => {
                    match state.analytics.instructors_registered().await {
                        Ok(no_of_instructors) => Ok(HttpResponse::Ok().json(no_of_instructors)),
                        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string()))
                    }
                }
                _ => Ok(HttpResponse::Forbidden().json("Access denied."))
            }
        }
        Err(e) => {
            Ok(HttpResponse::Unauthorized().json(e.to_string()))
        }
    }
}