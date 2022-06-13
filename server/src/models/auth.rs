use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::db::get_postgres;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Auth {
    id: String,
    account: i64,
    expire: i64,
}

impl Auth {
    pub async fn insert_auth_info(id: &str, account: i64, expire: i64) -> Result<(), sqlx::Error> {
        let pool = get_postgres();
        
        // delete overdue data
        let _ = sqlx::query(
            r#"delete from auth where id in (
            select id from auth where account = $1 order by expire asc limit 1
            ) and (select count(*) from auth where account = $1) >= 5;"#,
        )
        .bind(account)
        .execute(pool).await?;

        // insert new auth token
        let _ = sqlx::query("insert into auth (id, account, expire) values ($1, $2, $3);")
            .bind(id)
            .bind(account)
            .bind(expire)
            .execute(pool)
            .await?;
        Ok(())
    }
}
