use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(serde::Deserialize, FromRow, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StudentProfile {
    pub id: String,
    pub student_id: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub school_name: String,
    pub grade: i32,
}

#[derive(serde::Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StudentProfileNew {
    pub student_id: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
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