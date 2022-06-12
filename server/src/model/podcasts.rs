use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Content {
    pub id: i64,
    pub r#type: i32,
    pub title: String,
    pub source: String,
    pub author: i32,
    pub topic: i32,
    pub description: String,
    pub cover_image: String,
    pub up_date: sqlx::types::chrono::NaiveDate,
}