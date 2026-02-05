use std::sync::Arc;
use crate::subjects::repo::subject_repo::SubjectRepository;

pub struct SubjectsState {
    pub repo: Arc<dyn SubjectRepository + Send + Sync>
}