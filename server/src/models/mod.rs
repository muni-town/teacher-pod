use sqlx::FromRow;

pub mod account;
pub mod data;
pub mod podcast;

#[derive(Debug, FromRow)]
pub struct CountQuery {
    count: u32,
}