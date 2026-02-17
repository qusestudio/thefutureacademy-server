use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Instructor {
    pub id: i32,
    #[serde(rename="cognitoId")]
    pub cognito_id: String,
    pub name: String,
    pub email: String,
    #[serde(rename="phoneNumber")]
    pub phone_number: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstructorNew {
    #[serde(rename="cognitoId")]
    pub cognito_id: String,
    pub name: String,
    pub email: String,
    #[serde(rename="phoneNumber")]
    pub phone_number: String
}
