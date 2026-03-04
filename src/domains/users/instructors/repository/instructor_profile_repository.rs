use crate::domains::users::instructors::models::instructor_profile::{
    InstructorProfile, InstructorProfileNew,
};

#[async_trait::async_trait]
pub trait InstructorProfileRepository {
    async fn create_instructor_profile(
        &self,
        new_instructor_profile: InstructorProfileNew,
    ) -> sqlx::Result<Option<InstructorProfile>, sqlx::Error>;
    async fn get_instructor_profile(
        &self,
        instructor_id: String,
    ) -> sqlx::Result<Option<InstructorProfile>, sqlx::Error>;
    async fn update_instructor_profile(
        &self,
        instructor_id: String,
        new_instructor_profile: InstructorProfileNew,
    ) -> sqlx::Result<bool, sqlx::Error>;
    async fn delete_instructor_profile(
        &self,
        instructor_id: String,
    ) -> sqlx::Result<bool, sqlx::Error>;
}
