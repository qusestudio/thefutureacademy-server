use std::io::Error;
use actix_web::web::Data;
use std::sync::Arc;
use crate::infrastructure::event_bus::event_bus::{Event, EventBus};
use crate::infrastructure::analytics::models::event_log::EventLog;

use tokio::sync::broadcast::Receiver;
use uuid::Uuid;
use crate::infrastructure::analytics::repo::analytics_repo::AnalyticsRepository;

pub struct AnalyticsService {
    pub repo: Arc<dyn AnalyticsRepository + Send + Sync>,
    pub event_bus: Data<EventBus>,
}

impl AnalyticsService {
    pub async fn instructors_registered(&self) -> sqlx::Result<u64, Error> {
        match self.repo.instructors_registered().await {
            Ok(no_of_instructors) => Ok(no_of_instructors),
            Err(e) => Err(Error::other(e.to_string()))
        }
    }
    pub async fn students_registered(&self) -> Result<u64, Error> {
        match self.repo.students_registered().await { 
            Ok(no_of_students) => Ok(no_of_students),
            Err(e) => Err(Error::other(e.to_string()))
        }
    }
    pub async fn students_enrolled(&self) -> Result<u64, Error> {
        match self.repo.event_frequency("student.enrolled").await {
            Ok(count) => Ok(count),
            Err(e) => {Err(Error::other(format!("Students enrolled error: {}", e)))}
        }
    }
    pub async fn analytics_events_handler(&self, mut receiver: Receiver<Event>) {
        log::info!("Analytics Service listening for events...");
        while let Ok(event) = receiver.recv().await {
            match event.clone() {
                Event::InstructorRegistered(notification) => {
                    log::info!("Analytics: instructor.registered notification received");
                    let event_log = EventLog {
                        id: Uuid::now_v7().to_string(),
                        user_id: notification.instructor_id,
                        event_type: event.clone().to_string(),
                        created_at: chrono::Utc::now(),
                    };
                    let _event_log = self.repo.log_event(event_log).await;
                }
                Event::InstructorProfileCreated(notification) => {
                    log::info!("Analytics: instructor_profile.created notification received");
                    let event_log = EventLog {
                        id: Uuid::now_v7().to_string(),
                        user_id: notification.instructor_id,
                        event_type: event.clone().to_string(),
                        created_at: chrono::Utc::now(),
                    };
                    let _event_log = self.repo.log_event(event_log).await;
                }
                Event::StudentRegistered(notification) => {
                    log::info!("Analytics: student.registered notification received");
                    let event_log = EventLog {
                        id: Uuid::now_v7().to_string(),
                        user_id: notification.student_id,
                        event_type: event.clone().to_string(),
                        created_at: chrono::Utc::now(),
                    };
                    let _event_log = self.repo.log_event(event_log).await;
                }
                Event::StudentProfileCreated(notification) => {
                    log::info!("Analytics: student_profile.created notification received");
                    let event_log = EventLog {
                        id: Uuid::now_v7().to_string(),
                        user_id: notification.student_id,
                        event_type: event.clone().to_string(),
                        created_at: chrono::Utc::now(),
                    };
                    let _event_log = self.repo.log_event(event_log).await;

                }
                Event::SubjectViewed(notification) => {
                    log::info!("Analytics: subject.viewed notification received");
                    let event_log = EventLog {
                        id: Uuid::now_v7().to_string(),
                        user_id: notification.user_id,
                        event_type: event.clone().to_string(),
                        created_at: chrono::Utc::now(),
                    };
                    let _event_log = self.repo.log_event(event_log).await;
                }
                Event::TopicViewed(notification) => {
                    log::info!("Analytics: topic.viewed notification received");
                    let event_log = EventLog {
                        id: Uuid::now_v7().to_string(),
                        user_id: notification.user_id,
                        event_type: event.clone().to_string(),
                        created_at: chrono::Utc::now(),
                    };
                    let _event_log = self.repo.log_event(event_log).await;
                }
                Event::LessonCreated(notification) => {
                    log::info!("Analytics: lesson.created notification received");
                    let event_log = EventLog {
                        id: Uuid::now_v7().to_string(),
                        user_id: notification.user_id,
                        event_type: event.clone().to_string(),
                        created_at: chrono::Utc::now(),
                    };
                    let _event_log = self.repo.log_event(event_log).await;
                }
                Event::LessonOpened(notification) => {
                    log::info!("Analytics: lesson.opened notification received");
                    let event_log = EventLog {
                        id: Uuid::now_v7().to_string(),
                        user_id: notification.user_id,
                        event_type: event.clone().to_string(),
                        created_at: chrono::Utc::now(),
                    };
                    let _event_log = self.repo.log_event(event_log).await;
                }
                Event::StudentEnrolled(notification) => {
                    log::info!("Analytics: student.enrolled notification received");
                    let event_log = EventLog {
                        id: Uuid::now_v7().to_string(),
                        user_id: notification.student_id,
                        event_type: event.clone().to_string(),
                        created_at: chrono::Utc::now(),
                    };
                    let _event_log = self.repo.log_event(event_log).await;
                }
                Event::HealthCheck(message) => {
                    log::info!(
                        "Analytics Service: Message received =>, \"{}\"",
                        message.message
                    );
                }
                _ => {}
            }
        }
    }
}
