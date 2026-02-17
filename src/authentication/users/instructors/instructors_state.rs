use std::sync::Arc;
use crate::authentication::users::instructors::repository::instructor_repo::InstructorRepository;

pub struct InstructorsState {
    pub repo: Arc<dyn InstructorRepository + Send + Sync>
}