use axum::{extract::Path, response::Json, Extension};
use sqlx::PgPool;

use crate::{error::AppError, model::users::SimpleUser};

pub async fn get_user(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<SimpleUser>, AppError> {
    let user = sqlx::query_as::<_, SimpleUser>(SimpleUser::SELECT_FROM_ID)
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(Json(user))
}
