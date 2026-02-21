use crate::payments::models::checkout::{Checkout};

#[async_trait::async_trait]
pub trait CheckoutRepository {
    async fn get_checkout(&self, id: &str) -> sqlx::Result<Option<Checkout>, sqlx::Error>;
    async fn create_checkout(&self, new_checkout: Checkout) -> sqlx::Result<Checkout, sqlx::Error>;
    async fn update_checkout_status(&self, status: &str, checkout_id: &str) -> sqlx::Result<bool, sqlx::Error>;
}