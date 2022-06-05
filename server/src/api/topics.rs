use axum::{
    extract::{Path, Query},
    response::Json,
    Extension,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    error::AppError,
    model::{contents::Content, topics::Topic},
};

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

pub async fn popular_topics(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Topic>>, AppError> {
    let topics = sqlx::query_as::<_, Topic>(Topic::POPULAR_TOPICS)
        .fetch_all(&pool)
        .await?;
    Ok(Json(topics))
}

#[derive(Deserialize)]
pub struct RecommendInfo {
    id: i64,
}

pub async fn topic_recommend(
    info: Query<RecommendInfo>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Content>>, AppError> {
    let id = info.id;
    let contents = sqlx::query_as::<_, Content>(Content::LATEST_12_CONTENT_BY_TOPIC)
        .bind(id)
        .fetch_all(&pool)
        .await?;
    Ok(Json(contents))
}
