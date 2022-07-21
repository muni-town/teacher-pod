use std::time::Duration;

use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, PgPool};

static POSTGRES: OnceCell<PgPool> = OnceCell::new();

pub async fn init_pg_pool() {
    let db_connection_str = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can connect to database");
    POSTGRES.set(pool).unwrap();
}

#[inline]
pub fn get_postgres() -> &'static PgPool {
    unsafe { POSTGRES.get_unchecked() }
}
