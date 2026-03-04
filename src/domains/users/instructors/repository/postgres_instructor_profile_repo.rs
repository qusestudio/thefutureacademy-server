use sqlx::Error;
use uuid::Uuid;
use crate::domains::users::instructors::models::instructor_profile::{InstructorProfile, InstructorProfileNew};
use crate::domains::users::instructors::repository::instructor_profile_repository::InstructorProfileRepository;

pub struct PostgresInstructorProfileRepo {
    pub pg_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl InstructorProfileRepository for PostgresInstructorProfileRepo {
    async fn create_instructor_profile(&self, np: InstructorProfileNew) -> sqlx::Result<Option<InstructorProfile>, Error> {
        let id = Uuid::new_v4().to_string();
        let instructor_profile = sqlx::query_as(
            "INSERT INTO instructor_profiles
                (id, instructor_id, first_name, last_name, title, bio, profile_image_url, qualifications, years_of_experience, is_active, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                RETURNING *
                "
        )
            .bind(&id)
            .bind(&np.instructor_id)
            .bind(&np.first_name)
            .bind(&np.last_name)
            .bind(&np.title)
            .bind(&np.bio)
            .bind(&np.profile_image_url)
            .bind(&np.qualifications)
            .bind(np.years_of_experience)
            .bind(np.is_active)
            .bind(np.created_at)
            .bind(np.updated_at)
            .fetch_optional(&self.pg_pool)
            .await?;

        Ok(instructor_profile)
    }

    async fn get_instructor_profile(&self, instructor_id: String) -> sqlx::Result<Option<InstructorProfile>, Error> {
        let instructor_profile = sqlx::query_as("
            SELECT * FROM instructor_profiles WHERE instructor_id=$1
        ")
            .bind(&instructor_id)
            .fetch_optional(&self.pg_pool)
            .await?;

        Ok(instructor_profile)
    }

    async fn update_instructor_profile(&self, instructor_id: String, profile: InstructorProfileNew) -> sqlx::Result<bool, Error> {
        let old_instructor_profile = self.get_instructor_profile(instructor_id).await?;

        match old_instructor_profile {
            Some(old_instructor_profile) => {
                let _new_profile = sqlx::query(
                    "
                            UPDATE instructor_profiles
                            SET
                                first_name = $2,
                                last_name = $3,
                                title = $4,
                                bio = $5,
                                profile_image_url = $6,
                                qualifications = $7,
                                years_of_experience = $8,
                                is_active = $9,
                                updated_at = $10
                            WHERE instructor_id = $1
                        "
                )
                    .bind(&old_instructor_profile.instructor_id)
                    .bind(&profile.first_name)
                    .bind(&profile.last_name)
                    .bind(&profile.title)
                    .bind(&profile.bio)
                    .bind(&profile.profile_image_url)
                    .bind(&profile.qualifications)
                    .bind(&profile.years_of_experience)
                    .bind(&profile.is_active)
                    .bind(&profile.updated_at)
                    .execute(&self.pg_pool)
                    .await?;
                
                Ok(true)
            },
            None => {
                Ok(false)
            }
        }

    }

    async fn delete_instructor_profile(&self, instructor_id: String) -> sqlx::Result<bool, Error> {
        let old_instructor_profile = self.get_instructor_profile(instructor_id).await?;
        match old_instructor_profile {
            Some(old_instructor_profile) => {
                let _new_profile = sqlx::query("DELETE FROM instructor_profiles WHERE instructor_id=$1")
                    .bind(&old_instructor_profile.instructor_id)
                    .execute(&self.pg_pool)
                    .await?;
                Ok(true)
            },
            None => {
                Ok(false)
            }
        }
    }
}