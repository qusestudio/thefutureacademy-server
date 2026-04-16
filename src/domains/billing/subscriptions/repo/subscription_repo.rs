use crate::domains::billing::subscriptions::models::subscription::Subscription;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait SubscriptionRepository {
    async fn create_subscription(&self, s: Subscription) -> sqlx::Result<Subscription, sqlx::Error>;
    async fn get_subscription_by_student_id(&self, student_id: String) -> sqlx::Result<Subscription, sqlx::Error>;
    async fn get_subscriptions_for_student(&self, student_id: String) -> sqlx::Result<Vec<Subscription>, sqlx::Error>;
    async fn update_subscription_status(&self, status: String, student_id: String) -> sqlx::Result<bool, sqlx::Error>;
}