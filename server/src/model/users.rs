use std::iter::repeat_with;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub gender: String,
    pub email: String,
    pub reg_date: sqlx::types::chrono::NaiveDate,
    pub recently: sqlx::types::chrono::NaiveDate,
    pub password: String,
    pub salt: String,
    pub introduction: String,
    pub avatar: String,
    pub role: i32,
}

#[allow(dead_code)]
impl User {
    pub const INSERT_USER: &'static str = "insert into users 
        (username, email, password, salt, role) values 
        ($1, $2, $3, $4, $5);
    ";

    pub const SELECT_FROM_ID: &'static str = "select * from users where id = $1;";
    pub const SELECT_FROM_EMAIL: &'static str = "select * from users where email = $1;";

    pub fn generate_password(pwd: String) -> (String, String) {
        let salt: String = repeat_with(fastrand::alphanumeric).take(12).collect();
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
        (format!("{:x}", digest), salt)
    }
    pub fn check_password(input: &str, pwd: &str, salt: &str) -> bool {
        let digest = md5::compute(format!("@{salt}${input}").as_bytes());
        pwd == format!("{:x}", digest)
    }
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct SimpleUser {
    pub id: i64,
    pub username: String,
    pub gender: String,
    pub email: String,
    pub reg_date: sqlx::types::chrono::NaiveDate,
    pub introduction: String,
    pub avatar: String,
}

#[allow(dead_code)]
impl SimpleUser {
    pub const SELECT_FROM_ID: &'static str = "select id, username, gender, email, reg_date, introduction, avatar from users where id = $1";
}