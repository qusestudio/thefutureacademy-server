use crate::users::students::models::student::{Student, StudentNew};

#[async_trait::async_trait]
pub trait StudentRepository {
    async fn db_get_student_by_cognito(&self, cognito_id: &String) -> sqlx::Result<Student, sqlx::Error>;
    async fn db_create_student(&self, student_new: StudentNew) -> sqlx::Result<Student, sqlx::Error>;
}