use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Enrollment {
    pub id: String,
    pub student_id: String,
    pub subject_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnrollmentNew {
    #[serde(rename="studentId")]
    pub student_id: String,
    #[serde(rename="subjectId")]
    pub subject_id: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct StudentEnrollment {
    #[serde(rename = "studentId")]
    pub student_id: String,
    #[serde(rename = "subjectName")]
    pub student_name: String,
    #[serde(rename = "subjectId")]
    pub subject_id: String,
    #[serde(rename = "subjectTitle")]
    pub subject_title: String,
    pub grade: i32,
    pub term: i32,
}

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct AvailableSubject {
    #[serde(rename = "subjectId")]
    pub subject_id: String,
    #[serde(rename = "subjectTitle")]
    pub subject_title: String,
    pub grade: i32,
    pub term: i32,
}


impl Enrollment {
    pub fn new(enrollment_new: EnrollmentNew) -> Enrollment {
        Self {
            id: Uuid::new_v4().to_string(),
            student_id: enrollment_new.student_id,
            subject_id: enrollment_new.subject_id,
        }
    }
}