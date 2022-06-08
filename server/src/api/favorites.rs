use axum::{Extension, Json, extract::Path};
use sqlx::PgPool;

use crate::{error::AppError, model::state};

pub async fn favorite_state(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<state::Favorites>, AppError> {
    todo!()
}