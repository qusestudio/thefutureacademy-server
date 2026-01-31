use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub id: i32,
    #[serde(rename="cognitoId")]
    pub cognito_id: String,
    pub name: String,
    pub email: String,
    pub phone_number: String
}