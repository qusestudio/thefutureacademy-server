use std::sync::Arc;
use crate::users::students::repository::student_repo::StudentRepository;

pub struct StudentsState {
    pub repo: Arc<dyn StudentRepository + Send + Sync>
}