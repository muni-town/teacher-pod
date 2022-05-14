use axum::{extract::Path, response::Json, Extension};
use sqlx::PgPool;

use crate::{model::users::SimpleUser, error::AppError};

pub async fn get_user(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<SimpleUser>, AppError> {
    let user = sqlx::query_as::<_, SimpleUser>(
        "select 
        id, username, nickname, gender, email, reg_date, introduction, avatar 
        from users where id = $1"
    ).bind(id).fetch_one(&pool).await?;
    Ok(Json(user))
}