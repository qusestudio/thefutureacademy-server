use std::io::Error;
use std::sync::Arc;
use actix_web::web;
use crate::domains::users::students::models::student::Student;
use crate::domains::users::students::repository::student_repo::StudentRepository;
use crate::infrastructure::event_bus::event_bus::EventBus;

pub struct StudentsService {
    pub repo: Arc<dyn StudentRepository + Send + Sync>,
    pub event_bus: web::Data<EventBus>,
}

impl StudentsService {
    pub async fn get_student_by_cognito_id(&self, cognito_id: &String) -> Result<Student, Error> {
        let student = self.repo.db_get_student_by_cognito(cognito_id).await;
        match student {
            Ok(student) => Ok(student),
            Err(error) => {
                log::info!("Error getting student by cognito id: {:?}", error);
                Err(Error::other(error))
            }
        }
    }
    
    pub async fn get_all_students(&self) -> Result<Vec<Student>, Error> {
        let students = self.repo.db_get_all_students().await;
        match students {
            Ok(students) => Ok(students),
            Err(error) => {
                log::info!("Error getting all students: {:?}", error);
                Err(Error::other(error))
            }
        }
    }
}