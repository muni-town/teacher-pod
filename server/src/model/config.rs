use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub id: i64,
    pub name: String,
    pub value: String,
}

impl Config {
    pub const GET_CONFIG: &'static str = "select * from config where name = $1;";
}