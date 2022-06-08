use std::iter::repeat_with;

use axum::{
    extract::{Form, Query},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    auth::{self, AuthClaims},
    error::AppError,
    model::{
        auth::Auth as AuthModal,
        users::{SimpleUser, User},
    },
};

use super::OperResult;

pub async fn self_info(
    claims: AuthClaims,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<SimpleUser>, AppError> {
    let user_id = claims.user;
    let r = sqlx::query_as::<_, SimpleUser>(SimpleUser::SELECT_FROM_ID)
        .bind(user_id)
        .fetch_one(&pool)
        .await?;
    Ok(Json(r))
}

#[derive(Deserialize, Debug)]
pub struct UserRegisterForm {
    username: String,
    email: String,
    password: String,
}

pub async fn register(
    Form(info): Form<UserRegisterForm>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<OperResult>, AppError> {
    let (password, salt) = User::generate_password(info.password);

    let user_role = 1;

    sqlx::query(User::INSERT_USER)
        .bind(info.username)
        .bind(info.email)
        .bind(password)
        .bind(salt)
        .bind(user_role)
        .execute(&pool)
        .await?;

    Ok(Json(OperResult::ok()))
}

#[derive(Deserialize)]
pub struct UserLoginQuery {
    email: String,
    password: String,
}

pub async fn login(
    query: Query<UserLoginQuery>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = sqlx::query_as::<_, User>(User::SELECT_FROM_EMAIL)
        .bind(query.email.clone())
        .fetch_one(&pool)
        .await;
    let user = user?;

    if User::check_password(&query.password, &user.password, &user.salt) {
        let now = chrono::Local::now().timestamp();
        let expire = now + 60 * 60 * 24 * 2;
        let auth_id: String = repeat_with(fastrand::alphanumeric).take(12).collect();
        let token = auth::encode(&AuthClaims {
            exp: expire,
            iat: now,
            id: auth_id.clone(),
            user: user.id,
        });
        sqlx::query(AuthModal::INSER_AUTH)
            .bind(auth_id)
            .bind(user.id)
            .bind(expire)
            .execute(&pool)
            .await?;
        return Ok(Json(serde_json::json!({
            "token": token,
            "expire": expire,
        })));
    }
    return Err(AppError::Custom((
        StatusCode::BAD_REQUEST,
        "user password check failed".into(),
    )));
}
