use std::sync::Arc;
use crate::users::instructors::repository::instructor_repo::InstructorRepository;

pub struct InstructorsState {
    pub repo: Arc<dyn InstructorRepository + Send + Sync>
}