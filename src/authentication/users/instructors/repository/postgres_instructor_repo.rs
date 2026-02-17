use sqlx::{Error, PgPool};
use crate::authentication::users::instructors::models::instructor::{Instructor, InstructorNew};
use crate::authentication::users::instructors::repository::instructor_repo::InstructorRepository;

pub struct PostgresInstructorRepo {
    pub pg_pool: PgPool,
}

#[async_trait::async_trait]
impl InstructorRepository for PostgresInstructorRepo {
    async fn db_get_instructor_by_cognito(&self, cognito_id: &String) -> sqlx::Result<Instructor, Error> {
        let instructor = sqlx::query_as("SELECT * FROM instructors WHERE cognito_id = $1")
            .bind(cognito_id)
            .fetch_one(&self.pg_pool)
            .await?;

        Ok(instructor)
    }

    async fn db_create_instructor(&self, instructor_new: InstructorNew) -> sqlx::Result<Instructor, Error> {
        let instructor = sqlx::query_as(
            "INSERT INTO instructors (cognito_id, name, email, phone_number) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(&instructor_new.cognito_id)
        .bind(&instructor_new.name)
        .bind(&instructor_new.email)
        .bind(&instructor_new.phone_number)
        .fetch_one(&self.pg_pool)
        .await?;

        Ok(instructor)
    }
}
