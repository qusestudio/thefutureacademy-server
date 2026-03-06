use std::io::Error;
use std::sync::Arc;
use actix_web::web;
use crate::domains::enrollments::models::enrollment::{Enrollment, EnrollmentNew, StudentEnrolledEvent};
use crate::domains::enrollments::repo::enrollment_repo::EnrollmentRepository;
use crate::infrastructure::event_bus::event_bus::{Event, EventBus};

pub struct EnrollmentsService {
    pub repo: Arc<dyn EnrollmentRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}

impl EnrollmentsService {
    pub async fn create_enrollment(&self, user_id: &str, new_enrollment: EnrollmentNew) -> Result<Enrollment, Error> {
        match self.repo.db_create_enrollment(new_enrollment).await { 
            Ok(enrollment) => {
                match enrollment { 
                    Some(enrollment) => {
                        let student_enrolled = StudentEnrolledEvent::new(user_id);
                        let event = Event::StudentEnrolled(student_enrolled);
                        if let Err(e) = self.event_bus.send(event) {
                            log::error!("Failed to send student.enrolled event: {}", e);
                        };
                        Ok(enrollment)
                    }
                    None => Err(Error::other("Enrollment not found")),
                }
            }
            Err(error) => Err(Error::other(error.to_string()))
        }
    }
}