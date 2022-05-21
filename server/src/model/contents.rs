use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use super::users::SimpleUser;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Content {
    pub r#type: i32,
    pub title: String,
    pub source: String,
    pub author: i32,
    pub topic: i32,
    pub description: String,
    pub cover_image: String,
    pub up_date: sqlx::types::chrono::NaiveDate,
}

impl Content {
    pub const get_content:&'static str = "select * from contents where id = $1";
}