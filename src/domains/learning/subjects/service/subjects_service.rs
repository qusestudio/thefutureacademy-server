use std::io::Error;
use std::sync::Arc;
use actix_web::web;

use crate::domains::learning::subjects::models::subject::{Subject,  SubjectViewedEvent};
use crate::infrastructure::event_bus::event_bus::{Event, EventBus};
use crate::domains::learning::subjects::repo::subject_repo::SubjectRepository;


pub struct SubjectsService {
    pub repo: Arc<dyn SubjectRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}

impl SubjectsService {
    pub async fn get_subject(
        &self,
        user_id: &str,
        subject_id: &str,
    ) -> Result<Subject, Error> {
        match self.repo.db_get_subject(subject_id).await {
            Ok(subject) => {
                match subject {
                    Some(subject) => {
                        // emit event here.
                        let subject_viewed = SubjectViewedEvent::new(user_id, subject.title.as_str());
                        if let Err(e) = self.event_bus.send(Event::SubjectViewed(subject_viewed)) {
                            log::error!("Failed to send subject.viewed event: {}", e);
                        };
                        Ok(subject)
                    }
                    None => Err(Error::other(
                        "Subject not found",
                    )),
                }
            }
            Err(error) => Err(Error::other(error.to_string())),
        }
    }
}