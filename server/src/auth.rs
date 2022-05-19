use axum::{async_trait, extract::{FromRequest, RequestParts}, TypedHeader, headers::{Authorization, authorization::Bearer}};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub exp: i64,
    pub iat: i64,
    pub id: String,
    pub user: i64,
}

#[async_trait]
impl<B> FromRequest<B> for AuthClaims where B: Send {
    type Rejection = AppError;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AppError::InvalidToken)?;
        // Decode the user data
        let token_data = decode(bearer.token());
        if let None = token_data {
            return Err(AppError::InvalidToken);
        }
        Ok(token_data.unwrap())
    }
}

pub fn encode(claims: &AuthClaims) -> String {
    let secret = std::env::var("AUTH_SECRET").unwrap_or("TPENV".into());
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap_or_default()
}

pub fn decode(token: &str) -> Option<AuthClaims> {
    let secret = std::env::var("AUTH_SECRET").unwrap_or("TPENV".into());
    let r = jsonwebtoken::decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );
    if let Err(e) = r {
        return None;
    }
    Some(r.unwrap().claims)
}
