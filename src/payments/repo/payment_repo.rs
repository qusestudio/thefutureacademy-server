use crate::payments::models::payment::Payment;

#[async_trait::async_trait]
pub trait PaymentRepository {
    async fn create_payment(&self, payment: Payment) -> sqlx::Result<Option<Payment>, sqlx::Error>;
    async fn get_payment(&self, payment_id: String) -> sqlx::Result<Option<Payment>, sqlx::Error>;
    async fn update_payment(&self, payment: Payment) -> sqlx::Result<Payment, sqlx::Error>;
}