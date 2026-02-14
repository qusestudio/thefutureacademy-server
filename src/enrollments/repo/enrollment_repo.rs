use crate::enrollments::models::enrollment::{AvailableSubject, Enrollment, EnrollmentNew, StudentEnrollment};

#[async_trait::async_trait]
pub trait EnrollmentRepository {
    async fn db_get_enrollment(&self, id: &str) -> sqlx::Result<Option<Enrollment>, sqlx::Error>;
    async fn db_get_enrollments_by_student(&self, student_id: &str) -> sqlx::Result<Vec<StudentEnrollment>, sqlx::Error>;
    async fn db_get_available_subjects(&self, student_id: &str, grade: i32) -> sqlx::Result<Vec<AvailableSubject>, sqlx::Error>;
    async fn db_get_enrollments_by_subject(&self, subject_id: &str) -> sqlx::Result<Vec<Enrollment>, sqlx::Error>;
    async fn db_create_enrollment(&self, enrollment_new: EnrollmentNew) -> sqlx::Result<Enrollment, sqlx::Error>;
    async fn db_get_enrollment_with_subject_student(&self, subject_id: &str, student_id: &str) -> sqlx::Result<Enrollment, sqlx::Error>;
}