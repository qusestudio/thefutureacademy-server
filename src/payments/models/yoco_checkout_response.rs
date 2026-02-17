use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YocoCheckoutResponse {
    pub id: String,
    pub redirect_url: String,
    pub status: String,
    pub amount: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    pub merchant_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<LineItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    pub processing_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineItem {
    pub display_name: String,
    pub quantity: i32,
    pub pricing_details: PricingDetails,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingDetails {
    pub price: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i64>,
}