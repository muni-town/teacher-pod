use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Favorites {
    pub id: i64,
    pub account: i64,
    pub content: i64,
}

impl Favorites {
    pub const FAVORITES_STATE: &'static str = "select count(*) from favorites where account = $1 and content = $2;";
}