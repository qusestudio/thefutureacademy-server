use actix_web::{get, post};
use actix_web::web::{Json, Path};
use crate::users::students::models::student::{Student, StudentNew};
use crate::users::students::students_state::StudentsState;

#[get("/{cognito_id}")]
pub async fn get_student_by_cognito(state: actix_web::web::Data<StudentsState>, cognito_id: Path<String>) -> actix_web::Result<Json<Student>> {
    log::info!("Getting student with id {}", cognito_id.clone());

    let student = state
        .repo
        .db_get_student_by_cognito(&cognito_id.into_inner())
        .await
        .expect("Failed to get student");

    // todo: Proper Error Handling

    Ok(Json(student))
}

#[post("")]
pub async fn create_student(state: actix_web::web::Data<StudentsState>, payload: Json<StudentNew>) -> actix_web::Result<Json<Student>> {
    let student = state
        .repo
        .db_create_student(payload.into_inner())
        .await
        .expect("Failed creating student");

    Ok(Json(student))
}