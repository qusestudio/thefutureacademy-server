use std::sync::Arc;
use crate::contents::lessons::repo::lesson_repo::LessonRepository;

pub struct LessonsState {
    pub repo: Arc<dyn LessonRepository + Send + Sync>,
}