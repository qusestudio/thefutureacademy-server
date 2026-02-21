use std::sync::Arc;
use crate::payments::repo::payment_repo::PaymentRepository;

pub struct PaymentsState {
    pub repo: Arc<dyn PaymentRepository + Sync + Send>,
}