use crate::domains::billing::subscription::repo::subscription_repo::SubscriptionRepository;

pub struct PostgresSubscriptionRepo {
    pub pg_pool: sqlx::PgPool,
}

impl SubscriptionRepository for PostgresSubscriptionRepo {}