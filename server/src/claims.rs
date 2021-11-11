use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    pub static ref JWT_SECRET: String = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| generate_secret());
    #[derive(Copy, Clone, Debug)]
    pub static ref JWT_EXPIRATION_HOURS: i64 = std::env::var("JWT_EXPIRATION_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse::<i64>().expect("Expected a number in hours");
}

use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric};
pub fn generate_secret() -> String {
    let mut rng = thread_rng();
    let s: String = (&mut rng).sample_iter(Alphanumeric)
        .take(64) // secret length
        .map(char::from)
        .collect();
    dbg!(&s);
    return s;
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub username: String,
    pub permissions: Vec<String>,
    exp: i64,
}

impl Claims {
    pub fn new(username: String, permissions: Vec<String>) -> Self {
        Self {
            username,
            permissions,
            exp: (Utc::now() + Duration::hours(*JWT_EXPIRATION_HOURS)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub(crate) fn create_jwt(claims: Claims) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(JWT_SECRET.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}

/// Decode a json web token (JWT)
pub(crate) fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let decoding_key = DecodingKey::from_secret(JWT_SECRET.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}
