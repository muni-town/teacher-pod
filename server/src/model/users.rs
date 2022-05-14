use std::iter::repeat_with;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub gender: String,
    pub email: String,
    pub reg_date: i64,
    pub recently: i64,
    pub password: String,
    pub salt: String,
    pub introduction: String,
    pub avatar: String,
    pub role: i32,
}

impl User {
    pub const INSERT_USER: &'static str ="insert into users 
        (username, nickname, email, password, salt, role) values 
        ($1, $2, $3, $4, $5, $6);
    ";

    pub const SELECT_USER_FROM_ID: &'static str = "select * from users where id = ?;";
    pub const SELECT_USER_FROM_EMAIL: &'static str = "select * from users where email = ?;";
    pub const SELECT_USER_FROM_USERNAME: &'static str = "select * from users where username = ?;";

    pub fn generate_password(pwd: String) -> (String, String) {
        let salt: String = repeat_with(fastrand::alphanumeric).take(12).collect();
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
        (format!("{:x}", digest), salt)
    }
    pub fn check_password(input: String, pwd: String, salt: String) -> bool {
        let digest = md5::compute(format!("@{salt}${input}").as_bytes());
        pwd == format!("{:x}", digest)
    }
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
