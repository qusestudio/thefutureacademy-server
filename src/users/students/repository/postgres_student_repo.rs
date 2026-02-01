use crate::users::students::models::student::{Student, StudentNew};
use crate::users::students::repository::student_repo::StudentRepository;
use sqlx::{Error, PgPool};

pub struct PostgresStudentRepo {
    pub pg_pool: PgPool,
}

#[async_trait::async_trait]
impl StudentRepository for PostgresStudentRepo {
    async fn db_get_student_by_cognito(&self, cognito_id: &String) -> sqlx::Result<Student, Error> {
        let student = sqlx::query_as("SELECT * FROM students WHERE cognito_id = $1")
            .bind(cognito_id)
            .fetch_one(&self.pg_pool)
            .await?;

        Ok(student)
    }

    async fn db_create_student(&self, student_new: StudentNew) -> sqlx::Result<Student, Error> {
        let student = sqlx::query_as(
            "INSERT INTO students (cognito_id, name, email, phone_number) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(&student_new.cognito_id)
        .bind(&student_new.name)
        .bind(&student_new.email)
        .bind(&student_new.phone_number)
        .fetch_one(&self.pg_pool)
        .await?;

        Ok(student)
    }
}
