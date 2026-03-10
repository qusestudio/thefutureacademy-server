use crate::configuration::state::AppState;
use crate::domains::users::instructors::models::instructor::{Instructor, InstructorNew};
use crate::infrastructure::middleware::middleware::middleware;
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, get, post, web};

#[get("")]
pub async fn get_all_instructors(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            match claims.custom_role.as_str() {
                "admin" => {
                    match state.instructors.repo.db_get_all_instructors().await {
                        Ok(instructors) => {
                            log::info!("Admin user {} getting all instructors", claims.sub);
                            Ok(HttpResponse::Ok().json(instructors))
                        },
                        Err(err) => {
                            Ok(HttpResponse::InternalServerError().json(format!("{}", err)))
                        }
                    }
                },
                _ => Ok(HttpResponse::Forbidden().body("Access denied.")),
            }
        }
        Err(err) => {
            Ok(HttpResponse::Unauthorized().json(format!("{}", err)))
        }
    }
}

#[get("/{cognito_id}")]
pub async fn get_instructor_by_cognito(
    req: HttpRequest,
    state: web::Data<AppState>,
    cognito_id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("Getting instructor with id {}", &cognito_id.clone());

            match state
                .instructors
                .repo
                .db_get_instructor_by_cognito(&cognito_id.clone())
                .await
            {
                Ok(instructor) => Ok(HttpResponse::Ok().json(Json(instructor))),
                Err(_) => Ok(HttpResponse::NotFound().json("Instructor not found")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[post("")]
pub async fn create_instructor(
    state: web::Data<AppState>,
    payload: Json<InstructorNew>,
) -> actix_web::Result<Json<Instructor>> {
    let instructor = state
        .instructors
        .repo
        .db_create_instructor(payload.into_inner())
        .await
        .expect("Failed creating student");

    Ok(Json(instructor))
}
