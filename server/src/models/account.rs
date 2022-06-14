use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::db::get_postgres;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
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
    pub async fn query_from_id(id: i32) -> Result<Self, sqlx::Error> {
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


#[derive(FromRow, Serialize, Deserialize)]
pub struct Auth {
    id: String,
    account: i32,
    expire: i32,
}

impl Auth {
    pub async fn insert_auth_info(id: &str, account: i32, expire: i32) -> Result<(), sqlx::Error> {
        let pool = get_postgres();

        // delete overdue data
        let _ = sqlx::query(
            r#"delete from auth where id in (
            select id from auth where account = $1 order by expire asc limit 1
            ) and (select count(*) from auth where account = $1) >= 5;"#,
        )
        .bind(account)
        .execute(pool)
        .await?;

        // insert new auth token
        let _ = sqlx::query("insert into auth (id, account, expire) values ($1, $2, $3);")
            .bind(id)
            .bind(account)
            .bind(expire)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn check_auth_info(id: &str, account: i32) -> bool {
        let r = sqlx::query_as::<_, Auth>("select * from auth where id = $1 and account = $2;")
            .bind(id)
            .bind(account)
            .fetch_one(get_postgres())
            .await;
        if r.is_err() {
            false
        } else {
            true
        }
    }
}