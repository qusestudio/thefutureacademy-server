use crate::domains::learning::lessons::models::lesson::{Lesson, LessonCreatedEvent, LessonNew, LessonOpenedEvent};
use crate::domains::learning::lessons::repo::lesson_repo::LessonRepository;
use crate::infrastructure::event_bus::event_bus::{Event, EventBus};
use actix_web::web;
use std::io::Error;
use std::sync::Arc;

pub struct LessonsService {
    pub repo: Arc<dyn LessonRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}

impl LessonsService {
    pub async fn create_lesson(
        &self,
        user_id: &str,
        new_lesson: &LessonNew,
    ) -> Result<Lesson, Error> {
        match self.repo.db_create_lesson(new_lesson).await {
            Ok(lesson) => {
                match lesson {
                    Some(lesson) => {
                        // emit event here.
                        let lesson_created = LessonCreatedEvent::new(user_id, lesson.title.as_str());
                        if let Err(e) = self.event_bus.send(Event::LessonCreated(lesson_created)) {
                            log::error!("Failed to send lesson.created event: {}", e);
                        };
                        Ok(lesson)
                    }
                    None => Err(Error::other(
                        "Failed to create lesson or failed to return lesson.",
                    )),
                }
            }
            Err(error) => Err(Error::other(error.to_string())),
        }
    }

    pub async fn get_lesson(&self, user_id: &str, lesson_id: &str) -> Result<Lesson, Error> {
        match self.repo.db_get_lesson(lesson_id).await {
            Ok(lesson) => {
                match lesson {
                    Some(lesson) => {
                        // Emit event here.
                        let lesson_opened = LessonOpenedEvent::new(user_id, lesson.title.as_str());
                        if let Err(e) = self.event_bus.send(Event::LessonOpened(lesson_opened)) {
                            log::error!("Failed to lesson opened: {}", e);
                        };
                        Ok(lesson)
                    }
                    None => Err(Error::other("Lesson not found.")),
                }
            }
            Err(error) => Err(Error::other(error.to_string())),
        }
    }
    
    pub async fn get_lessons_by_topic_id(&self, topic_id: &str) -> Result<Vec<Lesson>, Error> {
        match self.repo.db_get_lessons_by_topic(topic_id).await {
            Ok(lessons) => {
                Ok(lessons)
            },
            Err(error) => Err(Error::other(error.to_string())),
        }
    }
    
}
