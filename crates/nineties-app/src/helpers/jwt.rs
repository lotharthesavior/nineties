use dotenv::dotenv;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT token claims payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// User ID (subject).
    pub sub: i32,
    /// Expiration timestamp (seconds since UNIX epoch).
    pub exp: usize,
}

static JWT_SECRET: Lazy<Vec<u8>> = Lazy::new(|| {
    dotenv().ok();
    env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set")
        .into_bytes()
});

static JWT_EXPIRY_HOURS: Lazy<u64> = Lazy::new(|| {
    dotenv().ok();
    env::var("JWT_EXPIRY_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse()
        .expect("Invalid JWT_EXPIRY_HOURS")
});

/// Returns the JWT signing secret (loaded once from the `JWT_SECRET` env var).
pub fn get_jwt_secret() -> &'static [u8] {
    &JWT_SECRET
}

/// Returns the JWT expiry duration in hours (loaded once from the `JWT_EXPIRY_HOURS` env var, default: 24).
pub fn get_jwt_expiry() -> u64 {
    *JWT_EXPIRY_HOURS
}

/// Creates a signed JWT token for the given user ID with the configured expiry.
pub fn create_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let exp = now_secs + (get_jwt_expiry() * 3600) as usize;
    let claims = Claims { sub: user_id, exp };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret()),
    )
}

/// Validates a JWT token and returns the user ID from its claims.
pub fn validate_token(token: &str) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data: TokenData<Claims> = decode(
        token,
        &DecodingKey::from_secret(get_jwt_secret()),
        &validation,
    )?;
    Ok(token_data.claims.sub)
}
