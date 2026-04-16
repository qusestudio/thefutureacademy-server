use crate::domains::billing::payments::models::payment::Payment;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait PaymentRepository {
    async fn create_payment(&self, payment: Payment) -> sqlx::Result<Option<Payment>, sqlx::Error>;
    async fn get_payment(&self, payment_id: String) -> sqlx::Result<Option<Payment>, sqlx::Error>;
    async fn update_payment(&self, payment: Payment) -> sqlx::Result<Payment, sqlx::Error>;
}