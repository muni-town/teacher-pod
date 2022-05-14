use axum::{
    extract::{Form, Path, Query},
    http::StatusCode,
    response::Json,
    Extension,
};
use axum_database_sessions::AxumSession;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    error::AppError,
    model::users::{SimpleUser, User},
};

use super::OperResult;

pub async fn self_info(
    session: AxumSession,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<SimpleUser>, AppError> {
    if let Some(current_user_id) = session.get::<i64>("user-id").await {
        let user = sqlx::query_as::<_, SimpleUser>(
            "select 
            id, username, nickname, gender, email, reg_date, introduction, avatar 
            from users where id = $1",
        )
        .bind(current_user_id)
        .fetch_one(&pool)
        .await?;
        Ok(Json(user))
    } else {
        Err(AppError::AccessDenied)
    }
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

    let nickname = info.username.clone();
    let user_role = 1;

    sqlx::query(User::INSERT_USER)
        .bind(info.username)
        .bind(nickname)
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
    session: AxumSession,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<OperResult>, AppError> {
    let user = sqlx::query_as::<_, User>(User::SELECT_USER_FROM_EMAIL)
        .bind(query.email.clone())
        .fetch_one(&pool)
        .await?;
    session.set("user-id", user.id).await;
    session.set("user-info", user).await;
    Ok(Json(OperResult::ok()))
}
