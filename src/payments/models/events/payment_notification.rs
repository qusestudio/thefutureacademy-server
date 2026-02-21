use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct YocoPaymentNotification {
    pub created_date: String,
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub payload: PaymentPayload,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaymentPayload {
    pub amount: i64,
    pub created_date: String,
    pub currency: String,
    pub id: String,
    pub mode: String,
    pub status: String,
    #[serde(rename = "type")]
    pub payment_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<PaymentMethodDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodDetails {
    #[serde(rename = "type")]
    pub method_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CardDetails {
    pub expiry_month: i32,
    pub expiry_year: i32,
    pub masked_card: String,
    pub scheme: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_holder: Option<String>,
}