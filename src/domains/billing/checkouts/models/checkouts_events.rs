use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CheckoutCreatedEvent {
    pub student_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CheckoutCompletedEvent {
    pub checkout_id: String,
}