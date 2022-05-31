use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Topic {
    pub id: i64,
    pub name: String,
    pub image: String,
}

impl Topic {
    pub const SELECT_FROM_ID: &'static str = "select id, namne, image from topics where id = $1;";
}