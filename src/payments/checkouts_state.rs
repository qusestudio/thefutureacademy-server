use std::sync::Arc;
use crate::payments::repo::checkout_repository::CheckoutRepository;

pub struct CheckoutsState {
    pub repo: Arc<dyn CheckoutRepository + Send + Sync>,
}