use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::Json};

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    id: i64,
    username: String,
    nickname: String,
    gender: String,
    email: String,
    reg_date: i64,
    recently: i64,
    password: String,
    salt: String,
    introduction: String,
    avatar: String,
    role: i32,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct SimpleUser {
    id: i64,
    username: String,
    nickname: String,
    gender: String,
    email: String,
    reg_date: sqlx::types::chrono::NaiveDate,
    introduction: String,
    avatar: String,
}