use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct JWTClaims {
    pub sub: String,
    pub email_verified: bool,
    pub iss: String,
    #[serde(rename = "cognito:username")]
    pub cognito_username: String,
    pub origin_jti: String,
    pub aud: String,
    pub event_id: String,
    pub token_use: String,
    pub auth_time: i64,
    pub exp: i64,
    #[serde(rename = "custom:role")]
    pub custom_role: String,
    pub iat: i64,
    pub jti: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    #[serde(rename="cognitoId")]
    pub cognito_id: String,
    pub name: String,
    pub email: String,
    #[serde(rename="phoneNumber")]
    pub phone_number: String
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Jwk {
    pub alg: String,
    pub e: String,
    pub kid: String,
    pub kty: String,
    pub n: String,
    #[serde(rename = "use")]
    pub r#use: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwkSet {
    pub keys: Vec<Jwk>,
}