use sqlx::{Error, PgPool};
use crate::payments::models::checkout::{Checkout};
use crate::payments::repo::checkout_repository::CheckoutRepository;

pub struct PostgresCheckoutRepository {
    pub pg_pool: PgPool,
}

#[async_trait::async_trait]
impl CheckoutRepository for PostgresCheckoutRepository {
    async fn get_checkout(&self, id: &str) -> sqlx::Result<Checkout, Error> {
        todo!()
    }

    async fn create_checkout(&self, checkout: Checkout) -> sqlx::Result<Checkout, Error> {
        let checkout = sqlx::query_as("
        INSERT INTO checkouts (id, student_id, amount, status, month, year, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *")
        .bind(&checkout.id)
        .bind(&checkout.student_id)
        .bind(&checkout.amount)
        .bind(&checkout.status)
        .bind(&checkout.month)
        .bind(&checkout.year)
        .bind(&checkout.created_at)
        .bind(&checkout.updated_at)
        .fetch_one(&self.pg_pool)
        .await?;
        Ok(checkout)
    }

    async fn update_checkout(&self, checkout: Checkout, checkout_id: &str) -> sqlx::Result<Checkout, Error> {
        todo!()
    }
}