use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{self, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    pub static ref JWT_SECRET: String = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| generate_secret());
        #[derive(Copy, Clone, Debug)]
    pub static ref JWT_EXPIRATION_HOURS: i64 = std::env::var("JWT_EXPIRATION_HOURS")
        .unwrap_or_else(|_| "24".to_string()) // default to 1 day expiration time
        .parse::<i64>().expect("Expected a number in hours");
}

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
pub fn generate_secret() -> String {
    let mut rng = thread_rng();
    let s: String = (&mut rng)
        .sample_iter(Alphanumeric)
        .take(512 / 4) // secret length (512 bits / 4 [2 hex chars])
        .map(char::from)
        .collect();
    trace!("secret {:?}", s);
    s
}

// https://github.com/DDtKey/actix-web-grants/blob/main/examples/jwt-httpauth/src/claims.rs
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
    let header = Header::new(Algorithm::HS512); // sha512 algorithm
    let encoding_key = EncodingKey::from_secret(JWT_SECRET.as_bytes());
    jsonwebtoken::encode(&header, &claims, &encoding_key).map_err(|e| ErrorUnauthorized(e.to_string()))
}

/// Decode a json web token (JWT)
pub(crate) fn decode_jwt(token: &str) -> Result<Claims, Error> {
    // let header = decode_header(&token)?;
    // let algorithm = match header.alg {
    //     "HS256" => Algorithm::HS256,
    //     "HS384" => Algorithm::HS384,
    //     "HS512" => Algorithm::HS512
    // };
    let algorithm = Algorithm::HS512;
    let decoding_key = DecodingKey::from_secret(JWT_SECRET.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::new(algorithm))
        .map(|data| data.claims)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}
