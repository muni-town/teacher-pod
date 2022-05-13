use axum::{extract::Query, response::Json, Extension};
use serde::Deserialize;
use sqlx::{PgPool, Row};

use crate::{model::user::SimpleUser, error::AppError};

#[derive(Deserialize)]
pub struct GetUser {
    id: Option<i64>,
    username: Option<String>,
}

pub async fn get_user(
    Query(params): Query<GetUser>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<SimpleUser>, AppError> {

    if params.id.is_none() && params.username.is_none() {
        return Err(AppError::MissingParams("id / username".into()));
    }

    let query = if params.id.is_some() {
        sqlx::query_as::<_, SimpleUser>(
            "select id, username, nickname, gender, email, reg_date, introduction, avatar from users where id = $1"
        ).bind(params.id.unwrap())
    } else {
        sqlx::query_as::<_, SimpleUser>(
            "select id, username, nickname, gender, email, reg_date, introduction, avatar from users where username = $1"
        ).bind(params.username.unwrap())
    };
    let user = query.fetch_one(&pool).await?;
    Ok(Json(user))
}
