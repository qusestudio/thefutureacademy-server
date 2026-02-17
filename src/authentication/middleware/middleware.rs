use crate::authentication::middleware::claims::{JWTClaims, JwkSet};
use actix_web::HttpRequest;
use jsonwebtoken::errors::ErrorKind::ExpiredSignature;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use std::io::Error;

pub async fn verify_token(token: &str) -> Result<JWTClaims, jsonwebtoken::errors::Error> {
    let url = std::env::var("AWS_COGNITO_JWK_JSON")
        .expect("AWS_COGNITO_JWK_JSON not set or can't be read correctly"); // Update this URL

    let response = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .expect("Failed requesting JWK JSON")
        .error_for_status()
        .unwrap();

    let jwk_set: JwkSet = response.json().await.expect("Failed to JWK set.");

    let n = jwk_set.keys[0].n.as_str();
    let e = jwk_set.keys[0].e.as_str();

    let user_pool =
        std::env::var("AWS_COGNITO_USER_POOL_ID").expect("AWS_COGNITO_USER_POOL_ID must be set");
    let client_id =
        std::env::var("AWS_COGNITO_CLIENT_ID").expect("AWS_COGNITO_USER_POOL_ID must be set");

    let issuer = format!("https://cognito-idp.us-east-2.amazonaws.com/{user_pool}");

    let decoding_key = DecodingKey::from_rsa_components(n, e)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[client_id]);
    validation.set_issuer(&[issuer]);

    let token_data = decode::<JWTClaims>(token, &decoding_key, &validation)?;

    Ok(token_data.claims)
}

pub async fn middleware(req: HttpRequest) -> actix_web::Result<JWTClaims, Error> {
    match req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|t| t.to_str().ok())
    {
        Some(auth_header) => match auth_header.strip_prefix("Bearer ") {
            Some(token) => match verify_token(token).await {
                Ok(decoded_token) => Ok(decoded_token),
                Err(e) => match e.kind() {
                    ExpiredSignature => Err(Error::other("The token is expired")),
                    _ => Err(Error::other("Error with the provided token")),
                },
            },
            None => Err(Error::other("Token not found")),
        },
        None => Err(Error::other("Authorization header not set")),
    }
}
