use crate::users::instructors::models::instructor::{Instructor, InstructorNew};

#[async_trait::async_trait]
pub trait InstructorRepository {
    async fn db_get_instructor_by_cognito(&self, cognito_id: &String) -> sqlx::Result<Instructor, sqlx::Error>;
    async fn db_create_instructor(&self, instructor_new: InstructorNew) -> sqlx::Result<Instructor, sqlx::Error>;
}