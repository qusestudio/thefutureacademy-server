use std::sync::Arc;
use crate::contents::subjects::repo::subject_repo::SubjectRepository;

pub struct SubjectsState {
    pub repo: Arc<dyn SubjectRepository + Send + Sync>
}