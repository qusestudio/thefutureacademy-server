use actix_web::{get, post, web, HttpRequest, HttpResponse};
use actix_web::web::{Json, Path};
use crate::middleware::middleware::middleware;
use crate::users::students::models::student::{Student, StudentNew};
use crate::users::students::students_state::StudentsState;



#[get("/{cognito_id}")]
pub async fn get_student_by_cognito(req: HttpRequest, state: web::Data<StudentsState>, cognito_id: Path<String>) -> actix_web::Result<HttpResponse> {
    match middleware(req).await {
        Ok(claims) => {
            log::info!("Getting student with id {}", &cognito_id.clone());

            match state.repo.db_get_student_by_cognito(&cognito_id.clone()).await {
                Ok(student) => {
                    Ok(HttpResponse::Ok().json(Json(student)))
                }
                Err(_) => {
                    Ok(HttpResponse::NotFound().json("Student not found"))
                }
            }
        }
        Err(e) => {
            Ok(HttpResponse::Unauthorized().json(e.to_string()))
        }
    }
}

#[post("")]
pub async fn create_student(state: web::Data<StudentsState>, payload: Json<StudentNew>) -> actix_web::Result<Json<Student>> {
    let student = state
        .repo
        .db_create_student(payload.into_inner())
        .await
        .expect("Failed creating student");

    Ok(Json(student))
}