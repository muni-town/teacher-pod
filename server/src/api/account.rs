use axum::{
    extract::{Form, Query},
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
) -> Result<Json<SimpleUser>, AppError> {
    if let Some(current_user) = session.get::<User>("user-info").await {
        Ok(Json(SimpleUser {
            id: current_user.id,
            username: current_user.username,
            gender: current_user.gender,
            email: current_user.email,
            reg_date: current_user.reg_date,
            introduction: current_user.introduction,
            avatar: current_user.avatar,
        }))
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
    session: AxumSession,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<OperResult>, AppError> {
    let user = sqlx::query_as::<_, User>(User::SELECT_FROM_EMAIL)
        .bind(query.email.clone())
        .fetch_one(&pool)
        .await;
    let user = user?;

    if User::check_password(&query.password, &user.password, &user.salt) {
        session.set("user-info", user).await;
        return Ok(Json(OperResult::ok()));
    }
    return Err(AppError::Custom((
        StatusCode::BAD_REQUEST,
        "user password check failed".into(),
    )));
}
