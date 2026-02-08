use std::sync::Arc;
use crate::lessons::repo::lesson_repo::LessonRepository;

pub struct LessonsState {
    pub repo: Arc<dyn LessonRepository + Send + Sync>,
}