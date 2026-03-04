use crate::domains::users::instructors::models::instructor::{Instructor, InstructorNew};
use crate::infrastructure::middleware::middleware::middleware;
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, get, post, web};
use crate::configuration::state::AppState;

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
