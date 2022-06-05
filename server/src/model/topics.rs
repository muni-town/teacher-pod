use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Topic {
    pub id: i64,
    pub name: String,
    pub image: String,
}

impl Topic {
    pub const SELECT_FROM_ID: &'static str = "select id, name, image from topics where id = $1;";
    pub const POPULAR_TOPICS: &'static str = "select id, name, image from topics order by id desc limit 6;";
}