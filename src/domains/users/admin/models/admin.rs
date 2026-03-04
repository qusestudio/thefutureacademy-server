use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Admin {
    pub id: i32,
    pub cognito_id: String,
    pub name: String,
    pub email: String,
    pub phone_number: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdminNew {
    pub cognito_id: String,
    pub name: String,
    pub email: String,
    pub phone_number: String
}