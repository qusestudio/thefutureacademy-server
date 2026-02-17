use crate::authentication::users::students::models::student_profile::{StudentProfile, StudentProfileNew};

#[async_trait::async_trait]
pub trait StudentProfileRepository {
    async fn db_get_student_profile(&self, cognito_id: &String) -> sqlx::Result<Option<StudentProfile>, sqlx::Error>;
    async fn db_create_student_profile(&self, profile_new: StudentProfileNew) -> sqlx::Result<StudentProfile, sqlx::Error>;
}