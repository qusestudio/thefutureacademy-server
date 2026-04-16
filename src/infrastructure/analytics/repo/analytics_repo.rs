use crate::infrastructure::analytics::models::event_log::EventLog;
use crate::infrastructure::event_bus::event_bus::Event;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait AnalyticsRepository {
    async fn instructors_registered(&self) -> sqlx::Result<u64>;
    async fn students_registered(&self) -> sqlx::Result<u64>;
    async fn no_of_enrollments(&self) -> sqlx::Result<u64>;
    async fn log_event(&self, log: EventLog) -> sqlx::Result<Option<EventLog>>;
    async fn event_frequency(&self, event_type: &str) -> sqlx::Result<u64>;
}