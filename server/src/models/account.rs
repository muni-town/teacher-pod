use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::db::get_postgres;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Account {
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
impl Account {
    pub async fn query_from_id(id: i64) -> Result<Self, sqlx::Error> {
        Ok(
            sqlx::query_as::<_, Account>("select * from account where id = $1;")
                .bind(id)
                .fetch_one(get_postgres())
                .await?,
        )
    }

    pub async fn query_from_email(email: &str) -> Result<Self, sqlx::Error> {
        Ok(sqlx::query_as("select * from account where email = $1;")
            .bind(email)
            .fetch_one(get_postgres())
            .await?)
    }

    pub fn generate_password(pwd: String) -> (String, String) {
        let salt: String = std::iter::repeat_with(fastrand::alphanumeric).take(12).collect();
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
        (format!("{:x}", digest), salt)
    }
    
    pub fn check_password(input: &str, pwd: &str, salt: &str) -> bool {
        let digest = md5::compute(format!("@{salt}${input}").as_bytes());
        pwd == format!("{:x}", digest)
    }
}
