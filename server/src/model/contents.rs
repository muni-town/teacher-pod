use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use super::users::SimpleUser;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Content {
    pub r#type: i64,
    pub title: String,
    pub source: String,
    pub author: i64,
    pub topic: i64,
    pub description: String,
    pub cover_image: String,
    pub up_date: sqlx::types::chrono::NaiveDate,
}

impl Content {
    pub const get_content:&'static str = "select * from contents where id = $1";
}