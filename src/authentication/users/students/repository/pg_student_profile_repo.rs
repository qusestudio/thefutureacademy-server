use sqlx::{Error, PgPool};
use crate::authentication::users::students::models::student_profile::{StudentProfile, StudentProfileNew};
use crate::authentication::users::students::repository::student_profile_repo::StudentProfileRepository;

pub struct PGStudentProfileRepo {
    pub pg_pool: PgPool,
}

#[async_trait::async_trait]
impl StudentProfileRepository for PGStudentProfileRepo {
    async fn db_get_student_profile(&self, cognito_id: &String) -> sqlx::Result<Option<StudentProfile>, Error> {
        let profile = sqlx::query_as("SELECT * FROM student_profiles WHERE student_id = $1")
            .bind(cognito_id)
            .fetch_optional(&self.pg_pool)
            .await?;

        Ok(profile)
    }

    async fn db_create_student_profile(&self, profile_new: StudentProfileNew) -> sqlx::Result<StudentProfile, Error> {
        let profile_new = StudentProfile::new(profile_new);
        let profile = sqlx::query_as(
            "INSERT INTO student_profiles (id, student_id, first_name, last_name, school_name, date_of_birth, grade) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        )
            .bind(profile_new.id)
            .bind(&profile_new.student_id)
            .bind(&profile_new.first_name)
            .bind(&profile_new.last_name)
            .bind(&profile_new.school_name)
            .bind(&profile_new.date_of_birth)
            .bind(&profile_new.grade)
            .fetch_one(&self.pg_pool)
            .await?;

        Ok(profile)
    }
}