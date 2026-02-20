use crate::payments::models::checkout::{Checkout};

#[async_trait::async_trait]
pub trait CheckoutRepository {
    async fn get_checkout(&self, id: &str) -> sqlx::Result<Checkout, sqlx::Error>;
    async fn create_checkout(&self, new_checkout: Checkout) -> sqlx::Result<Checkout, sqlx::Error>;
    async fn update_checkout(&self, checkout: Checkout, checkout_id: &str) -> sqlx::Result<Checkout, sqlx::Error>;
}