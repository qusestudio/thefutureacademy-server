use crate::subjects::models::subject::{Subject, SubjectNew};

#[async_trait::async_trait]
pub trait SubjectRepository {
    // READERS
    async fn db_get_subject(&self, id: &String) -> sqlx::Result<Subject, sqlx::Error>;
    async fn db_get_subjects_by_instructor(&self, instructor_id: &String) -> sqlx::Result<Vec<Subject>, sqlx::Error>;
    async fn db_get_subjects_by_grade(&self, grade: i32) -> sqlx::Result<Vec<Subject>, sqlx::Error>;
    async fn db_get_subjects_by_term(&self, term: i32) -> sqlx::Result<Vec<Subject>, sqlx::Error>;
    async fn db_get_subjects_by_term_and_grade(&self, term: i32, grade: i32) -> sqlx::Result<Vec<Subject>, sqlx::Error>;
    // CREATOR
    async fn db_create_subject(&self, subject_new: &SubjectNew) -> sqlx::Result<Subject, sqlx::Error>;
    // UPDATER
    async fn db_update_subject(&self, id: String, subject_new: &SubjectNew) -> sqlx::Result<Subject, sqlx::Error>;
    // DELETER
    async fn db_delete_subject(&self, id: String) -> sqlx::Result<bool, sqlx::Error>;
}