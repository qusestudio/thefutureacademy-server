use actix_web::{get, post, web, HttpRequest, HttpResponse};
use actix_web::web::{Json, Path};
use crate::authentication::middleware::middleware::middleware;
use crate::authentication::users::instructors::instructors_state::InstructorsState;
use crate::authentication::users::instructors::models::instructor::{Instructor, InstructorNew};

#[get("/{cognito_id}")]
pub async fn get_instructor_by_cognito(req: HttpRequest, state: web::Data<InstructorsState>, cognito_id: Path<String>) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("Getting instructor with id {}", &cognito_id.clone());

            match state.repo.db_get_instructor_by_cognito(&cognito_id.clone()).await {
                Ok(instructor) => {
                    Ok(HttpResponse::Ok().json(Json(instructor)))
                }
                Err(_) => {
                    Ok(HttpResponse::NotFound().json("Instructor not found"))
                }
            }
        }
        Err(e) => {
            Ok(HttpResponse::Unauthorized().json(e.to_string()))
        }
    }
}

#[post("")]
pub async fn create_instructor(state: web::Data<InstructorsState>, payload: Json<InstructorNew>) -> actix_web::Result<Json<Instructor>> {
    let instructor = state
        .repo
        .db_create_instructor(payload.into_inner())
        .await
        .expect("Failed creating student");

    Ok(Json(instructor))
}