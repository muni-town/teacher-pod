use axum::{extract::Path, response::Json, Extension};
use sqlx::PgPool;

use crate::{error::AppError, model::topics::Topic};

pub async fn get_topic(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Topic>, AppError> {
    let user = sqlx::query_as::<_, Topic>(Topic::SELECT_FROM_ID)
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(Json(user))
}