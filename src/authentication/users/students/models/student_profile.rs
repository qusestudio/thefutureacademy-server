use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(serde::Deserialize, FromRow, Serialize, Debug, Clone)]
pub struct StudentProfile {
    pub id: String,
    #[serde(rename="studentId")]
    pub student_id: String,
    #[serde(rename="firstName")]
    pub first_name: String,
    #[serde(rename="lastName")]
    pub last_name: String,
    #[serde(rename="dateOfBirth")]
    pub date_of_birth: String,
    #[serde(rename="schoolName")]
    pub school_name: String,
    pub grade: i32,
}

#[derive(serde::Deserialize, Serialize, Debug, Clone)]
pub struct StudentProfileNew {
    #[serde(rename="studentId")]
    pub student_id: String,
    #[serde(rename="firstName")]
    pub first_name: String,
    #[serde(rename="lastName")]
    pub last_name: String,
    #[serde(rename="dateOfBirth")]
    pub date_of_birth: String,
    #[serde(rename="schoolName")]
    pub school_name: String,
    pub grade: i32,
}

impl StudentProfile {
    pub fn new(profile_new: StudentProfileNew) -> StudentProfile {
        Self {
            id: Uuid::new_v4().to_string(),
            student_id: profile_new.student_id,
            first_name: profile_new.first_name,
            last_name: profile_new.last_name,
            date_of_birth: profile_new.date_of_birth,
            school_name: profile_new.school_name,
            grade: profile_new.grade,
        }
    }
}