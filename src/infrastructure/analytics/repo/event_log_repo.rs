use crate::infrastructure::analytics::models::event_log::EventLog;
use crate::infrastructure::event_bus::event_bus::Event;

#[async_trait::async_trait]
pub trait EventLogRepository {
    async fn log_event(&self, log: EventLog) -> sqlx::Result<Option<EventLog>>;
    async fn event_frequency(&self, event_type: &str) -> sqlx::Result<u64>;
}