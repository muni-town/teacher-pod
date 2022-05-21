use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    error::AppError,
    model::{contents::Content, users::SimpleUser},
};

/// this struct is use to response to api request.
#[derive(Deserialize, Serialize)]
pub struct ResponseContent {
    pub r#type: i64,
    pub title: String,
    pub source: String,
    pub author: SimpleUser,
    pub topic: i64,
    pub description: String,
    pub cover_image: String,
    pub up_date: sqlx::types::chrono::NaiveDate,
}

pub async fn get_content(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<ResponseContent>, AppError> {
    let content = sqlx::query_as::<_, Content>(Content::get_content)
        .bind(id)
        .fetch_one(&pool)
        .await?;

    let author = sqlx::query_as::<_, SimpleUser>(SimpleUser::SELECT_FROM_ID)
        .bind(content.author)
        .fetch_one(&pool)
        .await?;

    Ok(Json(ResponseContent {
        author,
        r#type: content.r#type,
        title: content.title,
        source: content.source,
        topic: content.topic,
        description: content.description,
        cover_image: content.cover_image,
        up_date: content.up_date,
    }))
}
