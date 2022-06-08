use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Auth {
    id: String,
    account: i64,
    expire: i64,
}

#[allow(dead_code)]
impl Auth {
    pub const EXIST_AUTH: &'static str =
        "select 1 from auth where id = $1 and account = $2 and expire = $3 limit 1;";
    pub const INSER_AUTH: &'static str =
        "insert into auth (id, account, expire) values ($1, $2, $3)";
}
