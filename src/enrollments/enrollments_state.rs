use std::sync::Arc;
use crate::enrollments::repo::enrollment_repo::EnrollmentRepository;

pub struct EnrollmentsState {
    pub repo: Arc<dyn EnrollmentRepository + Send + Sync>,
}