use axum::{extract::Path, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    error::AppError,
    model::{config::Config, contents::Content, users::SimpleUser},
};

/// this struct is use to response to api request.
#[derive(Deserialize, Serialize)]
pub struct ResponseContent {
    pub id: i64,
    pub r#type: i32,
    pub title: String,
    pub source: String,
    pub author: SimpleUser,
    pub topic: i32,
    pub description: String,
    pub cover_image: String,
    pub up_date: sqlx::types::chrono::NaiveDate,
}

pub async fn get_content(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<ResponseContent>, AppError> {
    let content = sqlx::query_as::<_, Content>(Content::GET_CONTENT)
        .bind(id)
        .fetch_one(&pool)
        .await?;

    let author = sqlx::query_as::<_, SimpleUser>(SimpleUser::SELECT_FROM_ID)
        .bind(content.author as i64)
        .fetch_one(&pool)
        .await?;

    Ok(Json(ResponseContent {
        id: content.id,
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

pub async fn recommend_content(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Content>>, AppError> {
    // load data from config
    let recommend_info = sqlx::query_as::<_, Config>(Config::GET_CONFIG)
        .bind("RecommendList")
        .fetch_one(&pool)
        .await;
    match recommend_info {
        Ok(config) => {
            let val = config.value;
            let v = serde_json::from_str::<Vec<u32>>(&val);
        }
        Err(_) => {}
    }

    // for the default, we use latest 12 content to recommend.
    let contents = sqlx::query_as::<_, Content>(Content::LATEST_12_CONTENT)
        .fetch_all(&pool)
        .await?;
    Ok(Json(contents))
}
