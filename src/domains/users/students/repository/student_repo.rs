use crate::domains::users::students::models::student::{Student, StudentNew};
use crate::domains::users::students::models::student_profile::StudentProfile;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait StudentRepository {
    async fn db_get_student_by_cognito(&self, cognito_id: &String) -> sqlx::Result<Student, sqlx::Error>;
    async fn db_create_student(&self, student_new: StudentNew) -> sqlx::Result<Student, sqlx::Error>;
    async fn db_get_all_students(&self) -> sqlx::Result<Vec<Student>, sqlx::Error>;
}

