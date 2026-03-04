use crate::infrastructure::middleware::middleware::middleware;
use crate::domains::users::students::models::student::{Student, StudentNew};
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, get, post, web};
use crate::configuration::state::AppState;
use crate::domains::users::students::service::students_service::StudentsService;

#[get("/{cognito_id}")]
pub async fn get_student_by_cognito(
    req: HttpRequest,
    state: web::Data<AppState>,
    cognito_id: Path<String>,
) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("Getting student with id {}", &cognito_id.clone());

            match state
                .students
                .repo
                .db_get_student_by_cognito(&cognito_id.clone())
                .await
            {
                Ok(student) => Ok(HttpResponse::Ok().json(Json(student))),
                Err(_) => Ok(HttpResponse::NotFound().json("Student not found")),
            }
        }
        Err(e) => Ok(HttpResponse::Unauthorized().json(e.to_string())),
    }
}

#[post("")]
pub async fn create_student(
    state: web::Data<AppState>,
    payload: Json<StudentNew>,
) -> actix_web::Result<Json<Student>> {
    let student = state
        .students
        .repo
        .db_create_student(payload.into_inner())
        .await
        .expect("Failed creating student");

    Ok(Json(student))
}
