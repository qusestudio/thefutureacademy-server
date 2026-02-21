use crate::payments::models::payment::Payment;
use crate::payments::repo::payment_repo::PaymentRepository;
use sqlx::{Error, PgPool};

pub struct PostgresPaymentRepo {
    pub pg_pool: PgPool,
}

#[async_trait::async_trait]
impl PaymentRepository for PostgresPaymentRepo {
    async fn create_payment(&self, payment: Payment) -> sqlx::Result<Option<Payment>, Error> {
        let payment = sqlx::query_as(
            "
            INSERT INTO payments (payment_id, checkout_id, status, created_at, updated_at) 
            VALUES ($1, $2, $3, $4, $5) 
            RETURNING *",
        )
        .bind(payment.payment_id)
        .bind(payment.checkout_id)
        .bind(payment.status)
        .bind(payment.created_at)
        .bind(payment.updated_at)
        .fetch_optional(&self.pg_pool)
        .await?;

        Ok(payment)
    }

    async fn get_payment(&self, payment_id: String) -> sqlx::Result<Option<Payment>, Error> {
        todo!()
    }

    async fn update_payment(&self, payment: Payment) -> sqlx::Result<Payment, Error> {
        todo!()
    }
}
