use crate::domains::learning::subjects::models::subject::{Subject, SubjectNew};

#[async_trait::async_trait]
pub trait SubjectRepository {
    // READERS
    async fn db_get_subject(&self, id: &String) -> sqlx::Result<Subject, sqlx::Error>;
    async fn db_get_all_subjects(&self) -> sqlx::Result<Vec<Subject>, sqlx::Error>;
    async fn db_get_subjects_by_grade(&self, grade: i32) -> sqlx::Result<Vec<Subject>, sqlx::Error>;
    async fn db_get_subjects_by_term(&self, term: i32) -> sqlx::Result<Vec<Subject>, sqlx::Error>;
    async fn db_get_subjects_by_term_and_grade(&self, term: i32, grade: i32) -> sqlx::Result<Vec<Subject>, sqlx::Error>;
    // CREATOR
    async fn db_create_subject(&self, subject_new: &SubjectNew) -> sqlx::Result<Subject, sqlx::Error>;
    async fn db_delete_subjects(&self, ids: Vec<String>) -> sqlx::Result<u64, sqlx::Error>;
}