use std::sync::Arc;
use crate::users::students::repository::student_profile_repo::StudentProfileRepository;
use crate::users::students::repository::student_repo::StudentRepository;

pub struct StudentsState {
    pub repo: Arc<dyn StudentRepository + Send + Sync>
}

pub struct StudentProfilesState {
    pub repo: Arc<dyn StudentProfileRepository + Send + Sync>
}