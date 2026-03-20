use crate::infrastructure::analytics::models::event_log::EventLog;
use crate::infrastructure::analytics::repo::analytics_repo::{AnalyticsRepository};

pub struct PostgresAnalyticsRepository {
    pub pg_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl AnalyticsRepository for PostgresAnalyticsRepository {
    async fn instructors_registered(&self) -> sqlx::Result<u64> {
        let no_of_instructors = sqlx::query("SELECT COUNT(*) FROM students")
            .execute(&self.pg_pool)
            .await?;

        Ok(no_of_instructors.rows_affected())
    }

    async fn students_registered(&self) -> sqlx::Result<u64> {
        let no_of_students = sqlx::query("SELECT COUNT(*) FROM students")
            .execute(&self.pg_pool)
            .await?;

        Ok(no_of_students.rows_affected())
    }

    async fn no_of_enrollments(&self) -> sqlx::Result<u64> {
        let no_of_enrollments = sqlx::query("SELECT COUNT(*) FROM enrollments")
            .execute(&self.pg_pool)
            .await?;

        Ok(no_of_enrollments.rows_affected())
    }

    async fn log_event(&self, log: EventLog) -> sqlx::Result<Option<EventLog>> {
        let event = sqlx::query_as("
            INSERT INTO event_logs (id, user_id, event_type, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        ")
        .bind(&log.id)
        .bind(&log.user_id)
        .bind(&log.event_type)
        .bind(&log.created_at)
        .fetch_optional(&self.pg_pool)
        .await?;

        Ok(event)
    }

    async fn event_frequency(&self, event_type: &str) -> sqlx::Result<u64> {
        let result = sqlx::query("SELECT COUNT(*) as count FROM event_logs WHERE event_type=$1")
            .bind(event_type)
            .execute(&self.pg_pool)
            .await?;

        Ok(result.rows_affected())
    }
}
