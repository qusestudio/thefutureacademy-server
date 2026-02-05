use crate::subjects::models::subject::{Subject, SubjectNew};
use crate::subjects::repo::subject_repo::SubjectRepository;
use sqlx::{Error, PgPool};

pub struct PostgresSubjectRepo {
    pub pg_pool: PgPool,
}

#[async_trait::async_trait]
impl SubjectRepository for PostgresSubjectRepo {
    async fn db_get_subject(&self, id: &String) -> sqlx::Result<Subject, Error> {
        let subject = sqlx::query_as("SELECT * FROM subjects WHERE id = $1")
            .bind(&id)
            .fetch_one(&self.pg_pool)
            .await?;

        Ok(subject)
    }

    async fn db_get_subjects_by_instructor(
        &self,
        instructor_id: &String,
    ) -> sqlx::Result<Vec<Subject>, Error> {
        let subjects = sqlx::query_as("SELECT * FROM subjects WHERE instructor_id = $1")
            .bind(&instructor_id)
            .fetch_all(&self.pg_pool)
            .await?;

        Ok(subjects)
    }

    async fn db_get_subjects_by_grade(&self, grade: i32) -> sqlx::Result<Vec<Subject>, Error> {
        let subjects = sqlx::query_as("SELECT * FROM subjects WHERE grade = $1")
            .bind(grade)
            .fetch_all(&self.pg_pool)
            .await?;

        Ok(subjects)
    }

    async fn db_get_subjects_by_term(&self, term: i32) -> sqlx::Result<Vec<Subject>, Error> {
        let subjects = sqlx::query_as("SELECT * FROM subjects WHERE term = $1")
            .bind(term)
            .fetch_all(&self.pg_pool)
            .await?;

        Ok(subjects)
    }

    async fn db_get_subjects_by_term_and_grade(
        &self,
        term: i32,
        grade: i32,
    ) -> sqlx::Result<Vec<Subject>, Error> {
        let subjects = sqlx::query_as("SELECT * FROM subjects WHERE term = $1 AND grade = $2")
            .bind(term)
            .bind(grade)
            .fetch_all(&self.pg_pool)
            .await?;

        Ok(subjects)
    }

    async fn db_create_subject(&self, subject_new: &SubjectNew) -> sqlx::Result<Subject, Error> {
        let subject = Subject::new(subject_new);
        let subjects = sqlx::query_as(
            "INSERT INTO subjects (id, instructor_id, title, grade, term) VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(&subject.id)
            .bind(&subject.instructor_id)
            .bind(&subject.title)
            .bind(&subject.grade)
            .bind(&subject.term)
            .fetch_one(&self.pg_pool)
            .await?;

        Ok(subjects)
    }

    async fn db_update_subject(
        &self,
        id: String,
        subject_new: &SubjectNew,
    ) -> sqlx::Result<Subject, Error> {
        unimplemented!()
    }

    async fn db_delete_subject(&self, id: String) -> sqlx::Result<bool, Error> {
        unimplemented!()
    }
}
