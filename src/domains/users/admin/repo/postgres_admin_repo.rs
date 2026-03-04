use crate::domains::users::students::models::student::{Student, StudentNew};
use crate::domains::users::students::repository::student_repo::StudentRepository;
use sqlx::{Error, PgPool};
use crate::domains::users::admin::models::admin::{Admin, AdminNew};
use crate::domains::users::admin::repo::admin_repo::AdminRepository;

pub struct PostgresAdminRepo {
    pub pg_pool: PgPool,
}

#[async_trait::async_trait]
impl AdminRepository for PostgresAdminRepo {
    async fn db_get_admin_by_cognito(&self, cognito_id: &String) -> sqlx::Result<Admin, Error> {
        let student = sqlx::query_as("SELECT * FROM admins WHERE cognito_id = $1")
            .bind(cognito_id)
            .fetch_one(&self.pg_pool)
            .await?;

        Ok(student)
    }

    async fn db_create_admin(&self, admin_new: AdminNew) -> sqlx::Result<Admin, Error> {
        let student = sqlx::query_as(
            "INSERT INTO admins (cognito_id, name, email, phone_number) VALUES ($1, $2, $3, $4) RETURNING *",
        )
            .bind(&admin_new.cognito_id)
            .bind(&admin_new.name)
            .bind(&admin_new.email)
            .bind(&admin_new.phone_number)
            .fetch_one(&self.pg_pool)
            .await?;

        Ok(student)
    }
}
