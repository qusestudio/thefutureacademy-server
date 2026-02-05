use crate::subjects::models::subject::{Subject, SubjectNew};

#[async_trait::async_trait]
pub trait SubjectRepository {
    // READERS
    async fn get_subject(&self, id: &String) -> sqlx::Result<Subject, sqlx::Error>;
    async fn get_subjects_by_instructor(&self, instructor_id: &String) -> sqlx::Result<Vec<Subject>, sqlx::Error>;
    async fn get_subjects_by_grade(&self, grade: i32) -> sqlx::Result<Vec<Subject>, sqlx::Error>;
    async fn get_subjects_by_term(&self, term: i32) -> sqlx::Result<Vec<Subject>, sqlx::Error>;
    async fn get_subjects_by_term_and_grade(&self, term: i32, grade: i32) -> sqlx::Result<Vec<Subject>, sqlx::Error>;
    // CREATOR
    async fn create_subject(&self, subject_new: &SubjectNew) -> sqlx::Result<Subject, sqlx::Error>;
    // UPDATER
    async fn update_subject(&self, id: String, subject_new: &SubjectNew) -> sqlx::Result<Subject, sqlx::Error>;
    // DELETER
    async fn delete_subject(&self, id: String) -> sqlx::Result<bool, sqlx::Error>;
}