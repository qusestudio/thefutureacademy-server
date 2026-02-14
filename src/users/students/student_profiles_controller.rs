use crate::middleware::middleware::middleware;
use crate::users::students::students_state::{StudentProfilesState, };
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, get, post, web};
use crate::users::students::models::student_profile::{ StudentProfileNew};

#[get("/{cognito_id}")]
pub async fn get_student_profile_by_cognito(
    req: HttpRequest,
    state: web::Data<StudentProfilesState>,
    cognito_id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("Getting student profile for {}", &claims.cognito_username);

            match state
                .repo
                .db_get_student_profile(&cognito_id.clone())
                .await
            {
                Ok(student) => Ok(HttpResponse::Ok().json(Json(student))),
                Err(_) => Ok(HttpResponse::NotFound().json("Student Profile not found")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[post("")]
pub async fn create_student_profile(
    req: HttpRequest,
    state: web::Data<StudentProfilesState>,
    payload: Json<StudentProfileNew>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("Creating student profile for user {}", &claims.sub);
            match state
                .repo
                .db_create_student_profile(payload.into_inner())
                .await {
                Ok(profile) => Ok(HttpResponse::Ok().json(Json(profile))),
                Err(e) => {
                    log::error!("Failed to create student profile: {}", e);
                    Ok(HttpResponse::NotFound().json(e.to_string()))
                }
            }
        },
        Err(e) => {
            log::error!("Failed to create student profile: {}", e);
            Ok(HttpResponse::Unauthorized().json(e.to_string()))
        },
    }
}
