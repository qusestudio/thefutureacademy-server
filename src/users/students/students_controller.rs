use actix_web::get;
use actix_web::web::{Json, Path};
use crate::users::students::models::student::Student;

#[get("/{student_id}")]
pub async fn get_student(student_id: Path<i32>) -> actix_web::Result<Json<Student>> {
    log::info!("Getting student with id {}", student_id.clone());
    
    let student = Student {
        id: student_id.into_inner(),
        cognito_id: "user_zero_cognito_id".to_string(),
        name: "Sipho".to_string(),
        email: "siphodube@gmail.com".to_string(),
        phone_number: "".to_string()
    };
    
    Ok(Json(student))
}