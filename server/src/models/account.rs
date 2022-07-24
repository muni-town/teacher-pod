use salvo::async_trait;

use crate::db::get_postgres;

pub use tp_models::account::{Account, Auth};

use super::CountQuery;

#[async_trait]
pub trait AccountFn {
    async fn query_from_id(id: i32) -> Result<Account, sqlx::Error>;
    async fn query_from_email(email: &str) -> Result<Account, sqlx::Error>;

    async fn email_exists(email: &str) -> bool;
    async fn insert_new_user(
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<(), sqlx::Error>;

    fn generate_password(pwd: String) -> (String, String);
    fn check_password(input: &str, pwd: &str, salt: &str) -> bool;
}

#[async_trait]
impl AccountFn for Account {
    async fn query_from_id(id: i32) -> Result<Self, sqlx::Error> {
        Ok(
            sqlx::query_as::<_, Account>("select * from account where id = $1;")
                .bind(id)
                .fetch_one(get_postgres())
                .await?,
        )
    }

    async fn query_from_email(email: &str) -> Result<Self, sqlx::Error> {
        Ok(sqlx::query_as("select * from account where email = $1;")
            .bind(email)
            .fetch_one(get_postgres())
            .await?)
    }

    async fn email_exists(email: &str) -> bool {
        let r = sqlx::query_as::<_, CountQuery>("select count(id) from account where email = $1;")
            .bind(email)
            .fetch_one(get_postgres())
            .await;
        if let Ok(r) = r {
            return r.count != 0;
        }
        false
    }

    async fn insert_new_user(
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<(), sqlx::Error> {
        let pool = get_postgres();

        let (password, salt) = Self::generate_password(password.into());

        let _ = sqlx::query(
            "insert into account (email, username, password, salt) values ($1, $2, $3, $4);",
        )
        .bind(email)
        .bind(username)
        .bind(password)
        .bind(salt)
        .execute(pool)
        .await?;

        Ok(())
    }

    fn generate_password(pwd: String) -> (String, String) {
        let salt: String = std::iter::repeat_with(fastrand::alphanumeric)
            .take(12)
            .collect();
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
        (format!("{:x}", digest), salt)
    }

    fn check_password(input: &str, pwd: &str, salt: &str) -> bool {
        let digest = md5::compute(format!("@{salt}${input}").as_bytes());
        pwd == format!("{:x}", digest)
    }
}

#[async_trait]
pub trait AuthFn {
    async fn insert_auth_info(id: &str, account: i32, expire: i32) -> Result<(), sqlx::Error>;
    async fn check_auth_info(id: &str, account: i32) -> bool;
}

#[async_trait]
impl AuthFn for Auth {
    async fn insert_auth_info(id: &str, account: i32, expire: i32) -> Result<(), sqlx::Error> {
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

    async fn check_auth_info(id: &str, account: i32) -> bool {
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
