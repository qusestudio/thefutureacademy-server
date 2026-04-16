use crate::domains::billing::subscriptions::models::subscription::{
    Subscription, SubscriptionFromDB,
};
use crate::domains::billing::subscriptions::repo::subscription_repo::SubscriptionRepository;
use sqlx::Error;

pub struct PostgresSubscriptionRepo {
    pub pg_pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl SubscriptionRepository for PostgresSubscriptionRepo {
    async fn create_subscription(&self, s: Subscription) -> sqlx::Result<Subscription, Error> {
        let row: Option<SubscriptionFromDB> = sqlx::query_as(
            "INSERT INTO subscriptions (id, student_id, plan_id, status, current_period_start, current_period_end, cancel_at_period_end) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
        )
            .bind(&s.id)
            .bind(&s.student_id)
            .bind(&s.plan_id)
            .bind(&s.status.to_string())
            .bind(&s.current_period_start)
            .bind(&s.current_period_end)
            .bind(&s.cancel_at_period_end)
            .fetch_optional(&self.pg_pool)
            .await?;

        match row {
            None => {
                log::error!("Error: Creating subscription failed.");
                Err(Error::RowNotFound)
            }
            Some(row) => {
                let subscription = Subscription::from_db(row);
                Ok(subscription)
            }
        }
    }

    async fn get_subscription_by_student_id(
        &self,
        student_id: String,
    ) -> sqlx::Result<Subscription, Error> {
        let row: Option<SubscriptionFromDB> = sqlx::query_as(
            "
                    SELECT * FROM subscriptions
                    WHERE student_id = $1
                    AND current_period_start <= NOW()
                    AND current_period_end >= NOW()
                    ORDER BY current_period_end DESC
                    LIMIT 1
                    ",
        )
        .bind(&student_id)
        .fetch_optional(&self.pg_pool)
        .await?;

        match row {
            None => {
                log::error!(
                    "Error: Subscription not found for student_id: {}",
                    student_id
                );
                Err(Error::RowNotFound)
            }
            Some(row) => {
                let subscription = Subscription::from_db(row);
                Ok(subscription)
            }
        }
    }

    async fn get_subscriptions_for_student(
        &self,
        student_id: String,
    ) -> sqlx::Result<Vec<Subscription>, Error> {
        let rows: Vec<SubscriptionFromDB> =
            sqlx::query_as("SELECT * FROM subscriptions WHERE student_id = $1")
                .bind(&student_id)
                .fetch_all(&self.pg_pool)
                .await?;

        let subscriptions: Vec<Subscription> = rows
            .iter()
            .map(|row| Subscription::from_db(row.clone()))
            .collect();
        Ok(subscriptions)
    }
}
