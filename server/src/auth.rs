use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    exp: usize,
    iat: usize,
    id: String,
    user: usize,
}

pub fn encode(claims: &AuthClaims) {
    let secret = std::env::var("AUTH_SECRET").unwrap_or(String::from("TEACHERPOD"));
    let r = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    );
}
